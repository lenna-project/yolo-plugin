use candle_core::{DType, Device, Module, Tensor};
use candle_nn::VarBuilder;
use image::{imageops::crop, DynamicImage, GenericImageView, Rgba};
use imageproc::{
    drawing::{draw_hollow_rect_mut, draw_text_mut},
    rect::Rect,
};
use lenna_core::ImageProcessor;
use rusttype::{Font, Scale};

use crate::{
    detection::Detection,
    yolov8::model::{report_detect, Multiples},
    Config,
};

pub mod coco_classes;
pub mod model;

type ModelType = model::YoloV8;

pub struct Yolo {
    pub config: Config,
    pub model: ModelType,
    pub detections: Vec<Detection>,
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

impl Clone for Yolo {
    fn clone(&self) -> Self {
        Yolo {
            config: self.config.clone(),
            detections: self.detections.clone(),
            ..Default::default()
        }
    }
}

impl Yolo {
    pub fn model() -> ModelType {
        let weights: Vec<u8> = include_bytes!("../../assets/yolov8n.safetensors").to_vec();
        let dev = &Device::Cpu;
        let vb = VarBuilder::from_buffered_safetensors(weights, DType::F32, dev).unwrap();
        let model = ModelType::load(vb, Multiples::n(), coco_classes::NAMES.len()).unwrap();
        model
    }

    pub fn classes() -> Vec<String> {
        coco_classes::NAMES.iter().map(|s| s.to_string()).collect()
    }

    pub fn scale(_width: u32, _height: u32, abox: &crate::bbox::BBox) -> Rect {
        Rect::at(abox.x as i32, abox.y as i32).of_size(abox.w as u32, abox.h as u32)
    }

    pub fn detect_objects(
        &self,
        image: &Box<DynamicImage>,
    ) -> Result<Vec<Detection>, Box<dyn std::error::Error>> {
        let conf_threshold = 0.25;
        let iou_threshold = 0.5;

        let original_image = image;

        let (width, height) = {
            let w = original_image.width() as usize;
            let h = original_image.height() as usize;
            if w < h {
                let w = w * 640 / h;
                // Sizes have to be divisible by 32.
                (w / 32 * 32, 640)
            } else {
                let h = h * 640 / w;
                (640, h / 32 * 32)
            }
        };

        let image_t = {
            let img = original_image.resize_exact(
                width as u32,
                height as u32,
                image::imageops::FilterType::CatmullRom,
            );
            let data = img.to_rgb8().into_raw();
            Tensor::from_vec(
                data,
                (img.height() as usize, img.width() as usize, 3),
                &Device::Cpu,
            )?
            .permute((2, 0, 1))?
        };

        let image_t = (image_t.unsqueeze(0)?.to_dtype(DType::F32)? * (1. / 255.))?;
        let predictions = self.model.forward(&image_t)?.squeeze(0)?;

        let original_image: DynamicImage = *original_image.clone();

        let bboxes = report_detect(
            &predictions,
            original_image,
            width,
            height,
            conf_threshold,
            iou_threshold,
        )?;

        let mut detections = Vec::new();

        for (class, bboxes) in bboxes.iter().enumerate() {
            for bbox in bboxes {
                let confidence = bbox.confidence.into();
                let bbox = crate::bbox::BBox {
                    x: bbox.xmin.into(),
                    y: bbox.ymin.into(),
                    w: (bbox.xmax - bbox.xmin).into(),
                    h: (bbox.ymax - bbox.ymin).into(),
                };
                let detection = Detection {
                    class,
                    bbox,
                    confidence,
                };
                detections.push(detection);
            }
        }

        Ok(detections)
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

                let font = Vec::from(include_bytes!("../../assets/DejaVuSans.ttf") as &[u8]);
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
