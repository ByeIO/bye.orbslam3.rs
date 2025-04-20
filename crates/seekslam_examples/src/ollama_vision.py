# 测试ollama以及minicpm视觉模型
# uv run ollama_vision.py
# 使用docker中的模型

from ollama import Client

# 建立连接
client = Client(host='http://localhost:11434')

# 提示词
PROMPT = '图片里面有几个字母？描述相对无人机的位置关系。'

# 模型
MODEL1 = 'aleSuglia/qwen2-vl-2b-instruct-q4_k_m:latest'
MODEL2 = 'minicpm-v:latest'

# 运行视觉模型
with open('../assets/scene1.png', 'rb') as f:
    image_data = f.read()
    # stream = client.generate(model='aleSuglia/qwen2-vl-2b-instruct-q4_k_m:latest', prompt='图片里面有几个字母？描述相对无人机的位置关系。', images=[image_data], stream=True)
    stream = client.generate(model=MODEL2, prompt=PROMPT, images=[image_data], stream=True)
    print("请求已提交, 等待服务端响应中...")
    
    # 收集完整响应
    full_response = []
    
    # 启用流式输出
    for chunk in stream:
        print(chunk['response'], end='', flush=True)
        full_response.append(chunk['response'])
    
    # 将完整结果写入日志文件
    with open('../result/ollama_vision.log', 'a', encoding='utf-8') as log_file:
        # log_file.write('\n\n=== New Response ===\n')
        # log_file.write(f'Model: {MODEL2}\n')
        # log_file.write(f'Prompt: {PROMPT}\n')
        # log_file.write('Response:\n')
        log_file.write(''.join(full_response))
        log_file.write('\n')