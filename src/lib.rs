use exif::{Field, In, Tag, Value};
use image::{DynamicImage, GenericImageView, Rgba};
use imageproc::drawing::{draw_hollow_rect_mut, draw_text_mut};
use imageproc::rect::Rect;
use lenna_core::plugins::PluginRegistrar;
use lenna_core::ProcessorConfig;
use lenna_core::{core::processor::ExifProcessor, core::processor::ImageProcessor, Processor};
use rusttype::{Font, Scale};
use std::collections::HashSet;
use std::io::Cursor;
use tract_ndarray::Axis;
use tract_onnx::prelude::*;

pub mod bbox;
use bbox::BBox;
pub mod detection;
use detection::{nms_sort, Detection};

extern "C" fn register(registrar: &mut dyn PluginRegistrar) {
    registrar.add_plugin(Box::new(Yolo::default()));
}

lenna_core::export_plugin!(register);

fn sigmoid(a: &f32) -> f32 {
    1.0 / (1.0 + (-a).exp())
}

pub const SIZE: usize = 416;
pub const TINY_YOLOV2_ANCHOR_PRIORS: [f32; 10] = [
    1.08, 1.19, 3.42, 4.41, 6.63, 11.38, 9.42, 5.11, 16.62, 10.52,
];

pub type ModelType = SimplePlan<TypedFact, Box<dyn TypedOp>, Graph<TypedFact, Box<dyn TypedOp>>>;

#[derive(Clone)]
pub struct Yolo {
    config: Config,
    pub model: ModelType,
    pub detections: Vec<Detection>,
}

impl Yolo {
    pub fn model() -> ModelType {
        let data = include_bytes!("../assets/tinyyolov2-7.onnx");
        let mut cursor = Cursor::new(data);
        let model = tract_onnx::onnx()
            .model_for_read(&mut cursor)
            .unwrap()
            .with_input_fact(
                0,
                InferenceFact::dt_shape(f32::datum_type(), tvec!(1, 3, SIZE, SIZE)),
            )
            .unwrap()
            .into_optimized()
            .unwrap()
            .into_runnable()
            .unwrap();
        model
    }

    pub fn classes() -> Vec<String> {
        let collect = include_str!("../assets/voc.names")
            .to_string()
            .lines()
            .map(|s| s.to_string())
            .collect();
        collect
    }

    pub fn scale(width: u32, height: u32, abox: &BBox) -> Rect {
        let r = abox.scale_to_rect(width as i32, height as i32);
        Rect::at(r.0, r.1).of_size(r.2, r.3)
    }

    pub fn detect_objects(
        &self,
        image: &Box<DynamicImage>,
    ) -> Result<Vec<Detection>, Box<dyn std::error::Error>> {
        let image_rgb = image.to_rgb8();
        let resized = image::imageops::resize(
            &image_rgb,
            416,
            416,
            ::image::imageops::FilterType::Triangle,
        );
        let tensor: Tensor =
            tract_ndarray::Array4::from_shape_fn((1, 3, SIZE, SIZE), |(_, c, y, x)| {
                resized[(x as _, y as _)][c] as f32
            })
            .into();
        let result = self.model.run(tvec!(tensor)).unwrap();
        let result: tract_ndarray::ArrayView4<f32> =
            result[0].to_array_view::<f32>()?.into_dimensionality()?;
        let result = result.index_axis(Axis(0), 0);

        let threshold = 0.5;
        let num_classes = Self::classes().len();

        let mut detections: Vec<Detection> = Vec::new();

        for (cy, iy) in result.axis_iter(Axis(1)).enumerate() {
            for (cx, ix) in iy.axis_iter(Axis(1)).enumerate() {
                let d = ix;
                for b in 0..5 {
                    let channel = b * (num_classes + 5);
                    let tx = d[channel + 0];
                    let ty = d[channel + 1];
                    let tw = d[channel + 2];
                    let th = d[channel + 3];
                    let tc = d[channel + 4];

                    let x = (cx as f32 + sigmoid(&tx)) * 32.0 / SIZE as f32;
                    let y = (cy as f32 + sigmoid(&ty)) * 32.0 / SIZE as f32;

                    let w = tw.exp() * (TINY_YOLOV2_ANCHOR_PRIORS[b * 2]) * 32.0 / SIZE as f32;
                    let h = th.exp() * (TINY_YOLOV2_ANCHOR_PRIORS[b * 2 + 1]) * 32.0 / SIZE as f32;

                    let tc = sigmoid(&tc);
                    let mut max_prob = (0, 0.0);
                    for c in 0..(num_classes) {
                        let v = d[5 + c] * tc;
                        if v > max_prob.1 {
                            max_prob = (c, v);
                        }
                    }
                    if max_prob.1 > threshold {
                        let bbox = BBox {
                            x: x as f64,
                            y: y as f64,
                            w: w as f64,
                            h: h as f64,
                        };
                        detections.push(Detection {
                            class: max_prob.0,
                            bbox,
                            confidence: max_prob.1 * tc,
                        });
                    }
                }
            }
        }
        let detections = nms_sort(detections);
        Ok(detections)
    }
}

impl Default for Yolo {
    fn default() -> Self {
        Yolo {
            config: Config::default(),
            model: Self::model(),
            detections: Vec::new(),
        }
    }
}

impl ImageProcessor for Yolo {
    fn process_image(
        &self,
        image: &mut Box<DynamicImage>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let white = Rgba([0u8, 255u8, 0u8, 255u8]);
        let mut img = DynamicImage::ImageRgba8(image.to_rgba8());
        let (width, height) = img.dimensions();
        let mut detections = self.detections.to_vec();

        let font = Vec::from(include_bytes!("../assets/DejaVuSans.ttf") as &[u8]);
        let font = Font::try_from_vec(font).unwrap();

        let font_height = 15.0;
        let scale = Scale {
            x: font_height * 2.0,
            y: font_height,
        };

        let mut classes: Vec<usize> = Vec::new();
        detections.iter_mut().for_each(|d| {
            let class = d.class;
            if !classes.contains(&class) {
                let bbox = &mut d.bbox;
                let label = Self::classes()[d.class].to_string();
                let rect = Self::scale(width, height, bbox);
                draw_hollow_rect_mut(&mut img, rect, white);

                draw_text_mut(
                    &mut img,
                    Rgba([0u8, 200u8, 0u8, 255u8]),
                    rect.left() as u32,
                    rect.top() as u32,
                    scale,
                    &font,
                    &label,
                );
                classes.push(class);
            }
        });

        *image = Box::new(img);
        Ok(())
    }
}

impl ExifProcessor for Yolo {
    fn process_exif(&self, exif: &mut Box<Vec<Field>>) -> Result<(), Box<dyn std::error::Error>> {
        let classes: Vec<String> = self
            .detections
            .clone()
            .into_iter()
            .map(|d| Self::classes()[d.class].clone())
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();
        if classes.len() > 0 {
            let description = classes.join(" ");
            exif.push(Field {
                tag: Tag::ImageDescription,
                ifd_num: In::PRIMARY,
                value: Value::Ascii(vec![description.clone().into_bytes()]),
            });
        }

        Ok(())
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct Config {}

impl Default for Config {
    fn default() -> Self {
        Config {}
    }
}

impl Processor for Yolo {
    fn name(&self) -> String {
        "yolo-plugin".into()
    }

    fn title(&self) -> String {
        "Yolo".into()
    }

    fn author(&self) -> String {
        "Christian M <chriamue@gmail.com>".into()
    }

    fn description(&self) -> String {
        "Yolo object detection".into()
    }

    fn process(
        &mut self,
        config: ProcessorConfig,
        image: &mut Box<lenna_core::LennaImage>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.config = serde_json::from_value(config.config).unwrap();
        self.detections = self.detect_objects(&image.image).unwrap();
        self.process_exif(&mut image.exif).unwrap();
        self.process_image(&mut image.image).unwrap();
        Ok(())
    }

    fn default_config(&self) -> serde_json::Value {
        serde_json::to_value(Config::default()).unwrap()
    }
}

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
lenna_core::export_wasm_plugin!(Yolo);

#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
#[allow(non_camel_case_types)]
type lenna_yolo_plugin = Yolo;
#[cfg(feature = "python")]
lenna_core::export_python_plugin!(lenna_yolo_plugin);

#[cfg(test)]
mod tests {
    use super::*;
    use lenna_core::ProcessorConfig;

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn default() {
        let mut yolo = Yolo::default();
        let c = yolo.default_config();

        let config = ProcessorConfig {
            id: "yolo-plugin".into(),
            config: c,
        };
        assert_eq!(yolo.name(), "yolo-plugin");
        let mut img =
            Box::new(lenna_core::io::read::read_from_file("assets/dog.jpg".into()).unwrap());
        yolo.process(config.clone(), &mut img).unwrap();
        img.name = "dog_out".to_string();
        lenna_core::io::write::write_to_file(&img, image::ImageOutputFormat::Jpeg(80)).unwrap();

        let mut img =
            Box::new(lenna_core::io::read::read_from_file("assets/person.jpg".into()).unwrap());
        yolo.process(config, &mut img).unwrap();
        img.name = "person_out".to_string();
        lenna_core::io::write::write_to_file(&img, image::ImageOutputFormat::Jpeg(80)).unwrap();

        let mut fields = Box::new(Vec::new());
        assert!(yolo.process_exif(&mut fields).is_ok());
        assert_eq!(fields.len(), 1);
    }

    #[cfg(target_arch = "wasm32")]
    mod wasm {
        use super::*;
        use lenna_core::LennaImage;
        use wasm_bindgen_test::*;

        #[wasm_bindgen_test]
        fn default() {
            let mut yolo = Yolo::default();
            let config = ProcessorConfig {
                id: "yolo".into(),
                config: yolo.default_config(),
            };
            assert_eq!(yolo.name(), "yolo-plugin");
            let mut img = Box::new(LennaImage::default());
            yolo.process(config, &mut img).unwrap();
        }
    }
}
