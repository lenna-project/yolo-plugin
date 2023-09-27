use lenna_core::plugins::PluginRegistrar;
use lenna_core::ProcessorConfig;
use lenna_core::{core::processor::ExifProcessor, core::processor::ImageProcessor, Processor};

pub mod bbox;
pub mod detection;

#[cfg(feature = "yolo")]
pub mod yolo;

#[cfg(feature = "yolo")]
use crate::yolo::Yolo;

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
