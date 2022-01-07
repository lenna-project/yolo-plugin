use image::{DynamicImage, GenericImageView, Rgba};
use imageproc::drawing::draw_hollow_rect_mut;
use imageproc::rect::Rect;
use lenna_core::plugins::PluginRegistrar;
use lenna_core::ProcessorConfig;
use lenna_core::{core::processor::ExifProcessor, core::processor::ImageProcessor, Processor};
use std::io::Cursor;
use tract_ndarray::{ArrayBase, Axis, Dim, ViewRepr};
use tract_onnx::prelude::*;

extern "C" fn register(registrar: &mut dyn PluginRegistrar) {
    registrar.add_plugin(Box::new(Yolo::default()));
}

lenna_core::export_plugin!(register);

fn sigmoid(a: &f32) -> f32 {
    1.0 / (1.0 + (-a).exp())
}

type ModelType = SimplePlan<TypedFact, Box<dyn TypedOp>, Graph<TypedFact, Box<dyn TypedOp>>>;

#[derive(Clone)]
pub struct Yolo {
    config: Config,
    model: ModelType,
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
                InferenceFact::dt_shape(f32::datum_type(), tvec!(1, 3, 416, 416)),
            )
            .unwrap()
            .into_optimized()
            .unwrap()
            .into_runnable()
            .unwrap();
        model
    }

    pub fn labels() -> Vec<String> {
        let collect = include_str!("../assets/voc.names")
            .to_string()
            .lines()
            .map(|s| s.to_string())
            .collect();
        collect
    }

    pub fn scale(width: u32, height: u32, abox: [f32; 4]) -> Rect {
        let width = width as f32;
        let height = height as f32;
        let x = abox[0];
        let y = abox[1];
        let w = abox[2] - abox[0];
        let h = abox[3] - abox[1];

        Rect::at((width * x) as i32, (height * y) as i32)
            .of_size(((width * w).abs()) as u32, ((height * h).abs()) as u32)
    }
}

impl Default for Yolo {
    fn default() -> Self {
        Yolo {
            config: Config::default(),
            model: Self::model(),
        }
    }
}

impl ImageProcessor for Yolo {
    fn process_image(
        &self,
        image: &mut Box<DynamicImage>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let image_rgb = image.to_rgb8();
        let resized = image::imageops::resize(
            &image_rgb,
            416,
            416,
            ::image::imageops::FilterType::Triangle,
        );
        let tensor: Tensor =
            tract_ndarray::Array4::from_shape_fn((1, 3, 416, 416), |(_, c, y, x)| {
                resized[(x as _, y as _)][c] as f32 / 255.0
            })
            .into();
        let result = self.model.run(tvec!(tensor)).unwrap();
        let result: tract_ndarray::ArrayView4<f32> =
            result[0].to_array_view::<f32>()?.into_dimensionality()?;
        let result = result.index_axis(Axis(0), 0);

        let threshold = 25.0;
        let white = Rgba([255u8, 255u8, 255u8, 255u8]);
        let mut img = DynamicImage::ImageRgba8(image.to_rgba8());
        let (width, height) = img.dimensions();
        let num_classes = 20;
        for (cy, iy) in result.axis_iter(Axis(1)).enumerate() {
            for (cx, ix) in iy.axis_iter(Axis(1)).enumerate() {
                for b in 0..4 {
                    let channel = b * (num_classes + 5);
                    let tx = ix[channel + 0];
                    let ty = ix[channel + 1];
                    let tw = ix[channel + 2];
                    let th = ix[channel + 3];
                    let tc = ix[channel + 4];

                    let x = (cx as f32 + sigmoid(&tx)) * 32.0 / 416.0;
                    let y = (cy as f32 + sigmoid(&ty)) * 32.0 / 416.0;

                    let w = tw.exp();
                    let h = th.exp();

                    // let tc = sigmoid(&tc);
                    let mut max_prob = (0, 0.0);
                    for c in 0..(num_classes - 1) {
                        let v = ix[5 + c] * tc;
                        if v > max_prob.1 {
                            max_prob = (c, v);
                        }
                    }

                    if max_prob.1 > threshold {
                        let label = Self::labels()[max_prob.0].to_string();
                        println!("{}: {} {}x{} {}x{}", label, max_prob.1, x, y, w, h);
                        let rect = Self::scale(width, height, [x, y, w, h]);
                        draw_hollow_rect_mut(&mut img, rect, white);
                    }
                }
            }
        }
        *image = Box::new(img);
        Ok(())
    }
}

impl ExifProcessor for Yolo {}

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
        let mut c = yolo.default_config();

        let config = ProcessorConfig {
            id: "yolo-plugin".into(),
            config: c,
        };
        assert_eq!(yolo.name(), "yolo-plugin");
        let mut img =
            Box::new(lenna_core::io::read::read_from_file("assets/dog.jpg".into()).unwrap());
        yolo.process(config.clone(), &mut img).unwrap();
        img.name = "test".to_string();
        lenna_core::io::write::write_to_file(&img, image::ImageOutputFormat::Jpeg(80)).unwrap();

        let mut img =
            Box::new(lenna_core::io::read::read_from_file("assets/person.jpg".into()).unwrap());
        yolo.process(config, &mut img).unwrap();
        img.name = "person_test".to_string();
        lenna_core::io::write::write_to_file(&img, image::ImageOutputFormat::Jpeg(80)).unwrap();
    }
}
