const pkg = import("../pkg");
import Widget from "./Widget";
export const ui = Widget;
export const processor = pkg;
export const name = () => "yolo-plugin";
export const description = () => "Yolo object detection";
export const process = async (config, image) => {
  return import("../pkg").then((processor) => processor.process(config, image));
};
export const defaultConfig = async () => {
  return {};
};
