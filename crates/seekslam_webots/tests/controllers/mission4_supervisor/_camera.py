# 获取camera数据并进行处理
# 输入: webots机器人实例上下文
# 输出: webots机器人的camera画面的base64编码数据
#        并且另外保存图片到目录

#############################################
# 第三方库
from gradio_client import Client, handle_file
from PIL import Image, ImageDraw
import cv2
import numpy as np
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

class _Camera():
    def __init__(self, robot, save_folder = "images"):
        # 机器人实例
        self.robot = robot
        # 图像计数器
        self.image_counter = 0
        # 图像存储文件夹
        self.save_folder = save_folder
    
    def capture_image_and_return(self):
        """
        捕获图像并返回base64编码数据
        :param robot: Webots机器人实例
        :return: base64编码的图像数据
        """
        # 获取图像数据
        image_data = self.robot.camera.getImage()
        if image_data:
            # 转换为OpenCV格式
            width = self.robot.camera.getWidth()
            height = self.robot.camera.getHeight()
            image = np.frombuffer(image_data, dtype=np.uint8).reshape((height, width, 4))
            image_bgr = cv2.cvtColor(image, cv2.COLOR_BGRA2BGR)
            
            # 转换为base64
            _, buffer = cv2.imencode('.jpg', image_bgr)
            img_base64 = base64.b64encode(buffer).decode('utf-8')
            return img_base64
        return None
    
    def capture_image_and_save(self):
        """
        捕获并保存当前相机图像
        :param robot: Webots机器人实例
        :param save_folder: 图像保存目录
        :param image_counter: 图像计数器
        :return: (base64编码的图像数据, 新的图像计数器)
        """
        # 获取base64编码
        img_base64 = self.capture_image_and_return()
        
        if img_base64 is None:
            return (None, self.image_counter)
        
        # 解码base64并保存图像
        img_data = base64.b64decode(img_base64)
        np_arr = np.frombuffer(img_data, np.uint8)
        image_bgr = cv2.imdecode(np_arr, cv2.IMREAD_COLOR)
        
        # 确保目录存在
        os.makedirs(self.save_folder, exist_ok=True)
        
        # 保存图像
        filename = os.path.join(self.save_folder, f"img_{self.image_counter:04d}.jpg")
        cv2.imwrite(filename, image_bgr)
        print("图像", filename, "保存成功")
        
        # 计数器
        self.image_counter += 1
        
        return img_base64
        
    def capture_image_and_save_and_return(self):
        """
        捕获图像，保存到目录并返回base64编码
        :param robot: Webots机器人实例
        :param save_folder: 保存目录
        :param image_counter: 图像计数器
        :return: base64编码的图像数据
        """
        return self.capture_image_and_save()
    