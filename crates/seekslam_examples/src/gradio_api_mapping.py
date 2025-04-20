# 使用gradio的API上传图片或视频进行建图并获取建图结果glb文件

# 原始图片
IMG_PATH = ["../assets/scene1.png"]

# 结果保存
RESULT_PATH = "../result"
# 文件名称(glb模型)
RESULT_NAME = "gradio_api_mapping.glb"

# 建图网页
WEB_URL = "http://192.168.31.20:7862"

##############################
# 内置库
import shutil
import os
import time
# gradio客户端
from gradio_client import file
import gradio_client as gr
##############################

def upload_files(client, input_images=None, input_video=None):
    """上传文件到Gradio应用并返回目标目录"""
    try:
        # 清除现有输入
        client.predict(api_name="/clear_fields")
        
        # 上传图片
        if input_images:
            print("正在上传图片...")
            gradio_files = [file(img_path) for img_path in input_images]
            # 上传文件并获取返回的实际目录路径
            _, target_dir, _, _ = client.predict(
                input_video=None,
                input_images=gradio_files,
                api_name="/update_gallery_on_upload_1"
            )
            print(f"图片上传完成，服务器返回的目标目录: {target_dir}")
            
            # 从完整路径中提取目录名
            if target_dir.startswith('/private'):
                # 处理MacOS临时路径
                dir_name = os.path.basename(os.path.dirname(target_dir))
            else:
                dir_name = os.path.basename(target_dir)
                
            print(f"使用的目标目录名: {dir_name}")
            return dir_name
        
    except Exception as e:
        print(f"上传文件时出错: {str(e)}")
        return None

def process_mapping(client, target_dir):
    """处理建图并获取GLB结果"""
    try:
        print(f"开始处理建图，使用目标目录: {target_dir}")
        
        # 调用建图API
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
        
        # 处理返回值
        if isinstance(result, (list, tuple)):
            model3d = result[0]  # 第一个返回值应该是GLB文件路径
            status_msg = result[1] if len(result) > 1 else ""
            print("建图状态:", status_msg)
            print("model3d", model3d)
            return model3d
        return result
        
    except Exception as e:
        print(f"处理建图时出错: {str(e)}")
        return None

def save_glb_file(glb_path, save_path, filename):
    """复制GLB文件到目标目录"""
    try:
        if not os.path.exists(save_path):
            os.makedirs(save_path)
        
        full_path = os.path.join(save_path, filename)
        
        # 检查源文件是否存在
        if not os.path.exists(glb_path):
            print(f"错误: 源文件 {glb_path} 不存在")
            return None
        
        print(f"正在复制GLB文件从 {glb_path} 到 {full_path}")
        shutil.copy2(glb_path, full_path)
        
        print(f"GLB文件已保存到: {full_path}")
        return full_path
    
    except Exception as e:
        print(f"复制GLB文件时出错: {str(e)}")
        return None

def main():
    try:
        # 创建客户端连接
        print("正在连接Gradio服务器...")
        client = gr.Client(WEB_URL)
        print("连接服务器成功")
        
        # 1. 上传文件并获取服务器返回的实际目录
        target_dir = upload_files(client, input_images=IMG_PATH)
        if not target_dir:
            print("无法获取有效的目标目录")
            return
        
        # 2. 处理建图
        glb_result = process_mapping(client, target_dir)
        if not glb_result:
            print("建图处理失败")
            return
        
        # 3. 保存结果
        saved_path = save_glb_file(glb_result, RESULT_PATH, RESULT_NAME)
        if saved_path:
            print("处理完成，结果已保存")
        else:
            print("处理完成，但保存结果失败")
            
    except Exception as e:
        print(f"主程序出错: {str(e)}")

if __name__ == "__main__":
    main()