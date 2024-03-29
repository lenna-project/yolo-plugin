use image::{imageops::crop, DynamicImage, GenericImageView, Rgba};
use imageproc::drawing::{draw_hollow_rect_mut, draw_text_mut};
use imageproc::rect::Rect;
use lenna_core::ImageProcessor;
use rusttype::{Font, Scale};
use std::io::Cursor;
use tract_ndarray::Axis;
use tract_onnx::prelude::*;

use crate::bbox::BBox;
use crate::detection::{nms_sort, Detection};
use crate::Config;

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
    pub config: Config,
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
            .with_output_fact(0, Default::default())
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
        let result = self.model.run(tvec!(tensor.into())).unwrap();
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
        let mut img = DynamicImage::ImageRgba8(image.to_rgba8());
        let (width, height) = img.dimensions();
        let mut detections = self.detections.to_vec();
        match self.config.crop {
            Some(true) => match detections.get(0) {
                Some(&detection) => {
                    let bbox = Yolo::scale(width, height, &detection.bbox);
                    let cropped = crop(
                        &mut img,
                        bbox.left() as u32,
                        bbox.top() as u32,
                        bbox.width() as u32,
                        bbox.height() as u32,
                    );
                    img = DynamicImage::ImageRgba8(cropped.to_image());
                }
                _ => {}
            },
            _ => {
                let white = Rgba([0u8, 255u8, 0u8, 255u8]);

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
                            rect.left(),
                            rect.top(),
                            scale,
                            &font,
                            &label,
                        );
                        classes.push(class);
                    }
                });
            }
        }
        *image = Box::new(img);
        Ok(())
    }
}
