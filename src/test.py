from PIL import Image
from numpy import asarray
import lenna_yolo_plugin
print(lenna_yolo_plugin.description())

image = Image.open('assets/dog.jpg')
data = asarray(image)
print(data.shape)

config = lenna_yolo_plugin.default_config()
processed = lenna_yolo_plugin.process(config, data)
print(processed.shape)
Image.fromarray(processed).save('lenna_test_out.png')
