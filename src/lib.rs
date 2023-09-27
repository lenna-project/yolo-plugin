use std::collections::HashSet;

use exif::{Field, In, Tag, Value};
use lenna_core::plugins::PluginRegistrar;
use lenna_core::ProcessorConfig;
use lenna_core::{core::processor::ExifProcessor, core::processor::ImageProcessor, Processor};

pub mod bbox;
pub mod detection;

#[cfg(feature = "yolo")]
pub mod yolo;

#[cfg(feature = "yolov8")]
pub mod yolov8;

#[cfg(feature = "yolo")]
pub use crate::yolo::Yolo;

#[cfg(feature = "yolov8")]
pub use crate::yolov8::Yolo;

extern "C" fn register(registrar: &mut dyn PluginRegistrar) {
    registrar.add_plugin(Box::new(Yolo::default()));
}

#[cfg(feature = "plugin")]
lenna_core::export_plugin!(register);

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub crop: Option<bool>,
}

impl Default for Config {
    fn default() -> Self {
        Config { crop: None }
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
    use lenna_core::Processor;
    use lenna_core::ProcessorConfig;

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn default() {
        let mut yolo = Yolo::default();
        let mut c = yolo.default_config();
        c["crop"] = serde_json::Value::Bool(true);

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

        let c = yolo.default_config();

        let config = ProcessorConfig {
            id: "yolo-plugin".into(),
            config: c,
        };
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
