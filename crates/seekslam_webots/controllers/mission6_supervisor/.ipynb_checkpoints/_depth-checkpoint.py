# 深度估计模块
# 输入: 图片base64数据
# 输出: 深度图base64数据

DEPTH_SERVICE_URL = "http://10.8.8.20:7863/"

#############################################
# 第三方库
from gradio_client import Client, handle_file
from PIL import Image
# 内置库
import base64
import io
import os
import logging
import time
import requests
#############################################

# 配置日志
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

def check_depth_service_available():
    """检查深度估计服务是否可用"""
    try:
        response = requests.get(f"{DEPTH_SERVICE_URL}/", timeout=5)
        if response.status_code == 200:
            logger.info("深度估计服务连接成功")
            return True
        logger.error(f"深度估计服务返回状态码: {response.status_code}")
        return False
    except Exception as e:
        logger.error(f"连接深度估计服务失败: {str(e)}")
        return False

def depth_service(img_data_base64: str) -> str:
    """
    深度估计服务接口
    :param img_data_base64: 输入图片的base64编码数据
    :return: 深度图的base64编码数据
    """
    temp_img_path = "./temp_depth_input.png"
    
    # 检查服务是否可用
    if not check_depth_service_available():
        raise ConnectionError("深度估计服务不可用, 请检查服务是否运行或URL是否正确")
    
    # 初始化Gradio客户端
    client = Client(DEPTH_SERVICE_URL)
    
    # 将base64数据转换为临时图片文件
    img_bytes = base64.b64decode(img_data_base64)
    with open(temp_img_path, "wb") as f:
        f.write(img_bytes)
    
    # 调用深度估计接口
    logger.info("正在调用深度估计服务...")
    result = client.predict(
        image=handle_file(temp_img_path),
        api_name="/on_submit"
    )
    
    logger.info("深度估计服务调用成功，处理结果...")
    
    # 处理返回结果（可能包含多个文件路径）
    depth_img_path = None
    for item in result:
        if isinstance(item, list):
            # 取第二个文件作为深度图
            if len(item) > 0:
                depth_img_path = item[1]
                break
        elif isinstance(item, str) and item.endswith(('.png', '.jpg', '.jpeg')):
            depth_img_path = item
            break
    
    if not depth_img_path or not os.path.exists(depth_img_path):
        raise ValueError("深度估计服务未返回有效的深度图路径")
    
    # 读取深度图并转换为base64
    with open(depth_img_path, "rb") as f:
        depth_img_bytes = f.read()
    depth_img_base64 = base64.b64encode(depth_img_bytes).decode('utf-8')
    
    # 清理临时文件
    if os.path.exists(temp_img_path):
        os.remove(temp_img_path)
        
    return depth_img_base64

def depth_img_and_save(img_data_base64:str, output_dir: str = "depth_results") -> str:
    """
    保存深度图结果到文件
    :param depth_img_base64: 深度图的base64数据
    :param output_dir: 输出目录路径
    :return: base64数据
    """
    # 获取深度图
    depth_img_base64 = depth_service(img_data_base64)
    
    # 创建输出目录
    os.makedirs(output_dir, exist_ok=True)
    
    # 生成带时间戳的输出文件名
    timestamp = int(time.time())
    output_path = os.path.join(output_dir, f"depth_result_{timestamp}.png")
    
    # 保存深度图
    with open(output_path, "wb") as f:
        f.write(base64.b64decode(depth_img_base64))
    
    return depth_img_base64

########################################
########################################
def _test():
    """
    测试深度估计模块
    测试图片: test_depth.jpg
    """
    # 读取测试图片并转换为base64
    test_img_path = "./seekslam_apartment_1.png"
    if not os.path.exists(test_img_path):
        raise FileNotFoundError(f"测试图片不存在: {test_img_path}")
        
    with open(test_img_path, "rb") as f:
        img_data_base64 = base64.b64encode(f.read()).decode('utf-8')
    
    # 调用深度估计服务
    print("正在进行深度估计...")
    depth_img_base64 = depth_img_and_save(img_data_base64)
    
    # 保存结果
    print(f"\n深度估计完成! 结果已保存")


if __name__ == "__main__":
    _test()