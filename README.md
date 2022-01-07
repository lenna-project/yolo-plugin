# lenna-plugin-template
Lenna Plugin Template

## 🚴 Usage

### 🐑 Use `cargo generate` to Clone this Template

```
cargo install cargo-generate
```

[Learn more about `cargo generate` here.](https://github.com/ashleygwilliams/cargo-generate)

```
cargo generate --git https://github.com/lenna-project/lenna-plugin-template.git --name my-plugin
cd my-plugin
```

### 🛠️ Build with `wasm-pack build`

```
wasm-pack build
```

### 🔬 Test in Headless Browsers with `wasm-pack test`

```
wasm-pack test --node
```

## Sources:

The model is from [tiny-yolov2 on onnx](https://github.com/onnx/models/tree/master/vision/object_detection_segmentation/tiny-yolov2).

The class names are from [voc names on darknet](https://github.com/pjreddie/darknet/blob/master/data/voc.names).

### Paper
"YOLO9000: Better, Faster, Stronger" [arXiv:1612.08242](https://arxiv.org/pdf/1612.08242.pdf)
