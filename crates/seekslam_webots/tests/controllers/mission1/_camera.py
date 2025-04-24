# 获取camera数据并进行处理
# 输入: webots机器人实例上下文
# 输出: webots机器人的camera画面的base64编码数据
#        并且另外保存图片到目录

#############################################
# 第三方库
from gradio_client import Client, handle_file
from PIL import Image, ImageDraw
import cv2
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

class _Camera:
    def __init__(robot):
        self.robot = robot
    
    # 捕获图片并返回为base64数据
    def capture_image_and_return(self):
        pass

    # 捕获图片并保存到目录
    def capture_image_and_save(self, save_folder, image_counter):
        """
        捕获并保存当前相机图像
        :param save_folder: 图像保存目录
        :param image_counter: 图像计数器
        :return: 新的图像计数器
        """
        # 获取图像数据
        image_data = self.camera.getImage()
        if image_data:
            # 转换为OpenCV格式
            width = self.camera.getWidth()
            height = self.camera.getHeight()
            image = np.frombuffer(image_data, dtype=np.uint8).reshape((height, width, 4))
            image_bgr = cv2.cvtColor(image, cv2.COLOR_BGRA2BGR)
            
            # 保存图像
            filename = os.path.join(save_folder, f"img_{image_counter:04d}.jpg")
            cv2.imwrite(filename, image_bgr)
            image_counter += 1
        
        return image_counter