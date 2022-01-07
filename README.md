# lenna-plugin-template
Lenna Yolo Plugin

This plugin runs the yolo model to label objects in images.

## 🚴 Usage

You can test the lib by detecting objects in two images.

```sh
cargo run
```

The results are:

![dog](assets/dog_out.jpg)

![person](assets/person_out.jpg)


### 🔬 Test in Headless Browsers with `wasm-pack test`

```
wasm-pack test --node
```

## Sources:

The model is from [tiny-yolov2 on onnx](https://github.com/onnx/models/tree/master/vision/object_detection_segmentation/tiny-yolov2).

The class names are from [voc names on darknet](https://github.com/pjreddie/darknet/blob/master/data/voc.names).

### Paper
"YOLO9000: Better, Faster, Stronger" [arXiv:1612.08242](https://arxiv.org/pdf/1612.08242.pdf)
