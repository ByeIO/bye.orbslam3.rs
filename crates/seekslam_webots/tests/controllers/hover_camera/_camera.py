# 获取camera数据并进行处理
# 输入: webots机器人实例上下文
# 输出: webots机器人的camera画面的base64编码数据

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

