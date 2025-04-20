# 使用gradio的API调用实现OCR识别
# 图片左上角为原点

from gradio_client import Client, handle_file
from PIL import Image, ImageDraw
import os

# 连接服务器接口
client = Client("http://192.168.31.20:7861/")
result = client.predict(
    img=handle_file('../assets/scene1.png'),
    lang="en",
    api_name="/predict"
)
print(result)

# 解析API返回结果
output_image_path, ocr_data = result
boxes = ocr_data['boxes']
texts = ocr_data['texts']
scores = ocr_data['scores']

# 打开原始图片
original_image = Image.open('../assets/scene1.png')
draw = ImageDraw.Draw(original_image)

# 设置框和文字样式
box_color = (0, 0, 255)  # 蓝色框
text_color = (255, 0, 0)  # 红色文字
font_size = 20

# 遍历所有识别结果
for box, text, score in zip(boxes, texts, scores):
    # 绘制四边形框
    draw.polygon([(box[0][0], box[0][1]), 
                 (box[1][0], box[1][1]),
                 (box[2][0], box[2][1]),
                 (box[3][0], box[3][1])], 
                 outline=box_color)
    
    # 在框的左上角添加识别文本和置信度
    text_with_score = f"{text}({score:.2f})"
    draw.text((box[0][0], box[0][1] - 25), 
             text_with_score, 
             fill=text_color)

# 创建结果目录（如果不存在）
os.makedirs('../result', exist_ok=True)

# 保存处理后的图片
output_path = '../result/gradio_api_ocr.png'
original_image.save(output_path)
print(f"处理后的图片已保存到: {output_path}")