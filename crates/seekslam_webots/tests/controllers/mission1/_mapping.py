# 建图模块
# 输入: 列表-图片base64数据
# 输出: 三维模型glb格式: base64数据

# Gradio服务地址
OCR_SERVICE_URL = "http://192.168.31.20:7862/"

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
import shutil
#############################################

# 配置日志
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

def check_mapping_service_available():
    """检查建图服务是否可用"""
    try:
        response = requests.get(f"{OCR_SERVICE_URL}/", timeout=5)
        if response.status_code == 200:
            logger.info("建图服务连接成功")
            return True
        logger.error(f"建图服务返回状态码: {response.status_code}")
        return False
    except Exception as e:
        logger.error(f"连接建图服务失败: {str(e)}")
        return False

def process_images_to_3dmodel(images_base64: list) -> str:
    """
    将多张图片转换为3D模型
    :param images_base64: 图片base64编码数据列表
    :return: glb格式3D模型的base64数据
    """
    # 检查服务是否可用
    if not check_mapping_service_available():
        raise ConnectionError("建图服务不可用, 请检查服务是否运行或URL是否正确")
    
    try:
        # 初始化Gradio客户端
        client = Client(OCR_SERVICE_URL)
        logger.info("Gradio客户端初始化成功")
        
        # 创建临时目录存放图片
        temp_dir = "temp_images"
        if not os.path.exists(temp_dir):
            os.makedirs(temp_dir)
        
        # 将base64图片保存为临时文件
        image_paths = []
        for i, img_data in enumerate(images_base64):
            img_bytes = base64.b64decode(img_data)
            img_path = os.path.join(temp_dir, f"image_{i}.png")
            with open(img_path, "wb") as f:
                f.write(img_bytes)
            image_paths.append(img_path)
        
        # 调用建图API
        # 1. 上传文件到服务端
        gradio_files = [handle_file(img_path) for img_path in image_paths]
        _, target_dir, _, _ = client.predict(
            input_video=None,
            input_images=gradio_files,
            api_name="/update_gallery_on_upload_1"
        )
        
        # 2. 处理建图
        result = client.predict(
            target_dir=target_dir,
            conf_thres=50,
            frame_filter="All",
            mask_black_bg=False,
            mask_white_bg=False,
            show_cam=True,
            mask_sky=False,
            prediction_mode="Depthmap and Camera Branch",
            api_name="/gradio_demo"
        )
        
        # 获取GLB文件路径
        glb_path = result[0] if isinstance(result, (list, tuple)) else result
        print("本机glb路径: ", glb_path)
        
        # 读取GLB文件并转换为base64
        with open(glb_path, "rb") as f:
            glb_data = base64.b64encode(f.read()).decode('utf-8')
        
        # 清理临时文件
        for img_path in image_paths:
            if os.path.exists(img_path):
                os.remove(img_path)
        if os.path.exists(temp_dir):
            os.rmdir(temp_dir)
            
        return glb_data
        
    except Exception as e:
        logger.error(f"建图处理失败: {str(e)}")
        raise

def save_glb_base64(glb_base64: str, save_path: str):
    """
    保存base64格式的GLB文件到本地
    :param glb_base64: 输入GLB文件的base64数据
    :param save_path: 保存路径
    """
    try:
        # 确保目录存在
        # os.makedirs(os.path.dirname(save_path), exist_ok=True)
        
        # 解码并保存
        with open(save_path, "wb") as f:
            f.write(base64.b64decode(glb_base64))
            
        logger.info(f"GLB文件已保存到: {save_path}")
        return True
    except Exception as e:
        logger.error(f"保存GLB文件失败: {str(e)}")
        return False

def _test():
    """测试函数"""
    # 读取测试图片并转换为base64
    test_images = ["../../../../assets/scene1.png"]
    test_images_base64 = []

    if not test_images:
        raise ValueError("测试目录中没有找到图片文件")

    for image in test_images:
         with open(image, "rb") as f:
                test_images_base64.append(base64.b64encode(f.read()).decode('utf-8'))
    
    # 调用建图服务
    print("正在进行3D建图...")
    output_path = "output_model.glb"
    try:
        glb_base64 = process_images_to_3dmodel(test_images_base64)
        
        # 保存结果
        if save_glb_base64(glb_base64, output_path):
            print(f"\n测试完成! 3D模型已保存到: {output_path}")
        else:
            print("\n测试完成，但保存模型失败")
            
    except Exception as e:
        print(f"\n建图过程中出错: {str(e)}")

if __name__ == "__main__":
    _test()