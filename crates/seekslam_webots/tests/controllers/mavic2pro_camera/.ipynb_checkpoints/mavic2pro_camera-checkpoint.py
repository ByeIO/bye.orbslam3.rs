# 测试1Hz获取camera图像添加序号并保存到文件夹

# Webots Python代码：1Hz频率获取相机图像并添加序号保存到文件夹

from controller import Robot, Camera
import cv2
import os
import time

class ImageCaptureRobot(Robot):
    def __init__(self):
        super().__init__()
        # 初始化设备
        self.timestep = int(self.getBasicTimeStep())
        
        # 获取相机设备
        self.camera = self.getDevice('camera')
        self.camera.enable(self.timestep)
        
        # 图像保存设置
        self.save_folder = 'captured_images'
        self.image_counter = 0
        self.last_capture_time = 0
        self.capture_interval = 1000  # 1秒 = 1000毫秒
        
        # 创建保存目录
        if not os.path.exists(self.save_folder):
            os.makedirs(self.save_folder)
            print(f"已创建图像保存目录: {self.save_folder}")
    
    def run(self):
        print("开始以1Hz频率捕获并保存图像...")
        print(f"图像将保存到: {os.path.abspath(self.save_folder)}")
        
        while self.step(self.timestep) != -1:
            current_time = self.getTime() * 1000  # 转换为毫秒
            
            # 检查是否到达捕获间隔
            if current_time - self.last_capture_time >= self.capture_interval:
                self.last_capture_time = current_time
                self.capture_image()
    
    def capture_image(self):
        """捕获当前相机图像并保存"""
        # 获取图像数据
        image_data = self.camera.getImage()
        
        if image_data:
            # 转换为OpenCV格式 (BGRA)
            width = self.camera.getWidth()
            height = self.camera.getHeight()
            image = np.frombuffer(image_data, dtype=np.uint8).reshape((height, width, 4))
            
            # 转换为BGR格式 (去掉Alpha通道)
            image_bgr = cv2.cvtColor(image, cv2.COLOR_BGRA2BGR)
            
            # 添加序号文本
            # text = f"Frame: {self.image_counter:04d}"
            # cv2.putText(image_bgr, text, (10, 30), 
                       # cv2.FONT_HERSHEY_SIMPLEX, 1, (0, 255, 0), 2)
            
            # 保存图像
            filename = os.path.join(self.save_folder, f"img_{self.image_counter:04d}.jpg")
            cv2.imwrite(filename, image_bgr)
            
            print(f"已保存：：: {filename}")
            self.image_counter += 1

# 主程序
if __name__ == "__main__":
    import numpy as np  # 确保numpy已安装
    
    robot = ImageCaptureRobot()
    robot.run()