# 使用无头浏览器操作gradio网页实现上传图片获取建图结果

# ```xpath
# 1. 清除上次结果: <button class="lg secondary  svelte-1ixn6qd" id="component-17" style="flex-grow: 1;"> Clear</button>
# 2. 图片上传: <button tabindex="0" aria-label="Click to upload or drop files" aria-dropeffect="copy" class="svelte-1b742ao center boundedheight flex icon-mode"> <input aria-label="File upload" data-testid="file-upload" type="file" accept="file/*" multiple="" class="svelte-1b742ao"></button>
# 3. 三维重建: <button class="lg primary  svelte-1ixn6qd" id="component-16" style="flex-grow: 1;"> Reconstruct</button>
# 4. 判断结果状态: <span class="md svelte-7ddecg prose"><p>Reconstruction Success (1 frames). Waiting for visualization.</p></span>
# 5. 获取结果: <a download="glbscene_15_All_maskbFalse_maskwFalse_camTrue_skyFalse_predDepthmap_and_Camera_Branch.glb" href="http://192.168.31.20:7862/gradio_api/file=/tmp/gradio/a98964b946804ef804c727e1f17ff33088e284afa812ba3c4e56e1ae41222f3b/glbscene_15_All_maskbFalse_maskwFalse_camTrue_skyFalse_predDepthmap_and_Camera_Branch.glb"><button aria-label="下载" aria-haspopup="false" title="下载" class="svelte-1h72pol padded" style="color: var(--block-label-text-color); --bg-color: var(--block-background-fill);"> <div class="svelte-1h72pol small"><svg xmlns="http://www.w3.org/2000/svg" width="100%" height="100%" viewBox="0 0 32 32"><path fill="currentColor" d="M26 24v4H6v-4H4v4a2 2 0 0 0 2 2h20a2 2 0 0 0 2-2v-4zm0-10l-1.41-1.41L17 20.17V2h-2v18.17l-7.59-7.58L6 14l10 10l10-10z"></path></svg> </div></button></a>
# ```

# 原始图片
IMG_PATH = ["../assets/scene1.png"]

# 结果保存
RESULT_PATH = "../result"
# 文件名称(glb模型)
RESULT_NAME = "gradio_headless_mapping.glb"

# 建图网页
WEB_URL = "http://192.168.31.20:7862"

##############################
import os
from selenium import webdriver
from selenium.webdriver.common.by import By
from selenium.webdriver.support.ui import WebDriverWait
from selenium.webdriver.support import expected_conditions as EC
import time
from PIL import Image
import numpy as np
import io
##############################

# 模拟滚动到底部
def scroll_to_bottom(driver):
    last_height = driver.execute_script("return document.body.scrollHeight")
    while True:
        driver.execute_script("window.scrollTo(0, document.body.scrollHeight);")
        time.sleep(2)  # 等待内容加载
        new_height = driver.execute_script("return document.body.scrollHeight")
        if new_height == last_height:
            break
        last_height = new_height

# 拼接截图
def capture_full_page_screenshot_stich(driver, file_path):
    # 模拟滚动
    scroll_to_bottom(driver)

    # 获取页面总高度
    total_height = driver.execute_script("return document.body.scrollHeight")
    viewport_height = driver.execute_script("return window.innerHeight")

    # 滚动并截图每一部分
    screenshots = []
    current_position = 0

    while current_position < total_height:
        # 滚动到当前位置
        driver.execute_script(f"window.scrollTo(0, {current_position});")
        time.sleep(0.2)  # 等待滚动完成

        # 截图并转换为 PIL Image
        screenshot = driver.get_screenshot_as_png()
        img = Image.open(io.BytesIO(screenshot))
        screenshots.append(img)

        current_position += viewport_height

    # 计算最终图片尺寸
    final_img = Image.new("RGB", (screenshots[0].width, total_height))
    y_offset = 0

    # 拼接所有截图
    for img in screenshots:
        final_img.paste(img, (0, y_offset))
        y_offset += img.height

    # 保存最终图片
    final_img.save(file_path)

# 初始化浏览器接口
def setup_driver():
    """初始化无头浏览器驱动"""
    options = webdriver.ChromeOptions()
    # 无头模式
    options.add_argument('--headless')  
    options.add_argument('--disable-gpu')
    # 设置缩放比例为25%
    # options.add_argument('--force-device-scale-factor=0.25')  
    driver = webdriver.Chrome(options=options)
    driver.execute_script("document.body.style.zoom='25%'")  # 使用JavaScript设置缩放
    return driver

def upload_and_reconstruct(driver, url):
    """
    执行上传图片和三维重建的主要流程
    :param driver: 浏览器驱动
    :param url: gradio应用地址
    """
    try:
        # 访问gradio应用
        driver.get(url)
        print("检测链接有效")
        # 等待页面加载
        time.sleep(30)  
        print("加载gradio网页成功")

        # 保存截图
        capture_full_page_screenshot_stich(driver,"../result/gradio_headless_mapping-1.png")
        # driver.save_screenshot("../result/gradio_headless_mapping-1.png")

        # 1. 清除上次结果（使用 class + id + 文本匹配）
        clear_btn = WebDriverWait(driver, 20).until(
            EC.element_to_be_clickable(
                (By.XPATH, '//button[contains(@class, "lg") and contains(@class, "secondary") and contains(@id, "component-") and contains(text(), "Clear")]')
            )
        )
        driver.execute_script("arguments[0].click();", clear_btn)
        print("已清除上次结果")
        time.sleep(1)

        # 保存截图
        driver.save_screenshot("../result/gradio_headless_mapping-2.png")

        # 2. 上传图片文件（使用 class + aria-label）
        upload_input = WebDriverWait(driver, 20).until(
            EC.presence_of_element_located(
                (By.CSS_SELECTOR, 'input[type="file"][aria-label="File upload"].svelte-1b742ao')
            )
        )
        img_path = os.path.abspath(IMG_PATH[0])
        if not os.path.exists(img_path):
            raise FileNotFoundError(f"图片文件不存在: {img_path}")
        upload_input.send_keys(img_path)
        print(f"已上传图片: {img_path}")
        time.sleep(2)

        # 保存截图
        driver.save_screenshot("../result/gradio_headless_mapping-3.png")

        # 3. 点击三维重建按钮（使用 class + id + 文本匹配）
        recon_btn = WebDriverWait(driver, 20).until(
            EC.element_to_be_clickable(
                (By.XPATH, '//button[contains(@class, "lg") and contains(@class, "primary") and contains(@id, "component-") and contains(text(), "Reconstruct")]')
            )
        )
        driver.execute_script("arguments[0].click();", recon_btn)
        print("已提交建图请求")

        # 保存截图
        driver.save_screenshot("../result/gradio_headless_mapping-4.png")

        # 4. 等待重建完成（使用 class + 文本匹配）
        WebDriverWait(driver, 300).until(
            EC.text_to_be_present_in_element(
                (By.XPATH, '//span[contains(@class, "prose")]//p'),
                "Reconstruction Success"
            )
        )
        print("三维重建成功完成")

        # 保存截图
        driver.save_screenshot("../result/gradio_headless_mapping-5.png")

        # 5. 下载结果模型文件（使用 download 属性和 class）
        download_link = WebDriverWait(driver, 30).until(
            EC.presence_of_element_located(
                (By.CSS_SELECTOR, 'a[download$=".glb"].svelte-1h72pol')
            )
        )
        download_url = download_link.get_attribute('href')
        print(f"模型文件下载链接: {download_url}")
        return download_url

    except Exception as e:
        print(f"处理过程中出现错误: {str(e)}")
        driver.save_screenshot("../result/gradio_headless_mapping-error.png")
        raise

def main():
    """主函数"""
    # 创建结果目录
    os.makedirs(RESULT_PATH, exist_ok=True)
    
    # 初始化浏览器驱动
    driver = setup_driver()
    try:
        # 替换为实际的gradio应用地址
        gradio_url = WEB_URL
        upload_and_reconstruct(driver, gradio_url)
    finally:
        driver.quit()

if __name__ == "__main__":
    main()