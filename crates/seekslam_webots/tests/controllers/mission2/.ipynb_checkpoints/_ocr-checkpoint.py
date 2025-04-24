# OCR识别模块
# 输入: 图片base64数据
# 输出: [{'txt':'识别文本', 'box': [[x1,y1],[x2,y2],[x3,y3],[x4,y4]], 'score': 置信度 }, ...]

# 坐标系说明: 图片左上角为原点(0,0)，向右为x轴正方向，向下为y轴正方向

OCR_SERVICE_URL = "http://192.168.31.20:7861/"

#############################################
# 第三方库
from gradio_client import Client, handle_file
from PIL import Image, ImageDraw
# 内置库
import base64
import io
import os
import json
import requests
import logging
#############################################

# 配置日志
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

def check_ocr_service_available():
    """检查OCR服务是否可用"""
    try:
        response = requests.get(f"{OCR_SERVICE_URL}/", timeout=5)
        if response.status_code == 200:
            logger.info("OCR服务连接成功")
            return True
        logger.error(f"OCR服务返回状态码: {response.status_code}")
        return False
    except Exception as e:
        logger.error(f"连接OCR服务失败: {str(e)}")
        return False

def ocr_service(img_data_base64: str) -> list:
    """
    OCR识别服务接口
    :param img_data_base64: 图片的base64编码数据
    :return: 识别结果列表，每个元素包含文本、坐标框和置信度
    """
    temp_img_path = "./temp_ocr_input.png"
    print("hello_from_ocr_service")

    # 首先检查服务是否可用
    if not check_ocr_service_available():
        raise ConnectionError("OCR服务不可用, 请检查服务是否运行或URL是否正确")
    
    # 初始化Gradio客户端
    global client
    client = Client(OCR_SERVICE_URL)
    print("初始化gradio客户端成功")

    # 将base64数据转换为临时图片文件
    img_bytes = base64.b64decode(img_data_base64)
    with open(temp_img_path, "wb") as f:
        f.write(img_bytes)
    # exit(0)
    
    # 调用OCR接口
    logger.info("正在调用OCR服务...")
    result = client.predict(
        img=handle_file(temp_img_path),
        lang="en",  # 默认使用英文识别
        api_name="/predict"
    )
    
    logger.info("OCR服务调用成功，解析结果...")
    
    # 解析返回结果
    if not result or len(result) < 1:
        raise ValueError("OCR服务返回了无效的结果格式")
        
    _, ocr_data = result
    if not isinstance(ocr_data, dict):
        raise ValueError(f"OCR服务返回了意外的数据类型: {type(ocr_data)}")
    
    # 确保所有必要字段都存在
    required_fields = ['boxes', 'texts', 'scores']
    for field in required_fields:
        if field not in ocr_data:
            raise ValueError(f"OCR结果缺少必要字段: {field}")
    
    boxes = ocr_data['boxes']
    texts = ocr_data['texts']
    scores = ocr_data['scores']
    
    # 验证数据长度一致
    if not (len(boxes) == len(texts) == len(scores)):
        raise ValueError("OCR结果中boxes、texts和scores长度不一致")
    
    # 格式化输出结果
    output = []
    for box, text, score in zip(boxes, texts, scores):
        # 验证box格式
        if len(box) != 4 or any(len(point) != 2 for point in box):
            logger.warning(f"忽略无效的box格式: {box}")
            continue
            
        output.append({
            'txt': text,
            'box': [[float(point[0]), float(point[1])] for point in box],  # 转换为坐标点列表
            'score': float(score)
        })
    
    print("清理临时文件")
    # 清理临时文件
    if os.path.exists(temp_img_path):
        os.remove(temp_img_path)
        
    return output
    
def img_with_box(ocr_results: list, img_data_base64: str) -> str:
    """
    在图片上绘制OCR识别结果框和文本
    :param ocr_results: ocr_service返回的识别结果
    :param img_data_base64: 原始图片的base64数据
    :return: 带标注框的图片base64数据
    """
    # 将base64图片数据转换为PIL图像对象
    img_bytes = base64.b64decode(img_data_base64)
    img = Image.open(io.BytesIO(img_bytes))
    draw = ImageDraw.Draw(img)
    
    # 设置绘制样式
    box_color = (0, 0, 255)  # 蓝色框
    text_color = (255, 0, 0)  # 红色文字
    
    # 遍历所有识别结果并绘制
    for result in ocr_results:
        box = result['box']
        text = result['txt']
        score = result['score']
        
        # 绘制四边形框
        draw.polygon([
            (box[0][0], box[0][1]),
            (box[1][0], box[1][1]),
            (box[2][0], box[2][1]),
            (box[3][0], box[3][1])
        ], outline=box_color)
        
        # 在框上方添加识别文本和置信度
        text_with_score = f"{text}({score:.2f})"
        draw.text((box[0][0], box[0][1] - 25), 
                 text_with_score, 
                 fill=text_color)
    
    # 将处理后的图片转换为base64
    buffered = io.BytesIO()
    img.save(buffered, format="PNG")
    img_base64 = base64.b64encode(buffered.getvalue()).decode('utf-8')
    
    return img_base64

########################################
########################################
def _test():
    '''
    测试图片: ../../../../assets/scene1.png
    '''
    img_data_base64 = ''
    # 读取测试图片并转换为base64
    test_img_path = "../../../../assets/scene1.png"
    if not os.path.exists(test_img_path):
        raise FileNotFoundError(f"测试图片不存在: {test_img_path}")
        
    with open(test_img_path, "rb") as f:
        img_data_base64 = base64.b64encode(f.read()).decode('utf-8')
    
    # 调用OCR服务
    print("正在进行OCR识别...")
    ocr_results = ocr_service(img_data_base64)
    
    # 打印识别结果
    print("\n识别结果:")
    for i, result in enumerate(ocr_results, 1):
        print(f"{i}. 文本: {result['txt']}")
        print(f"   坐标: {result['box']}")
        print(f"   置信度: {result['score']:.2f}")
    
    # 生成带标注框的图片
    print("\n生成可视化结果...")
    annotated_img_base64 = img_with_box(ocr_results, img_data_base64)
    
    # 保存结果图片
    output_path = "ocr_result.png"
    with open(output_path, "wb") as f:
        f.write(base64.b64decode(annotated_img_base64))
    
    print(f"\n测试完成! 结果已保存到: {output_path}")
    
    # 尝试打开结果图片(仅当在支持的环境中)
    img = Image.open(output_path)
    img.show()

if __name__ == "__main__":
    _test()