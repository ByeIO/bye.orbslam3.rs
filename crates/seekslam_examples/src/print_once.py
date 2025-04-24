# 测试多轮循环只执行一次的函数

# 使用装饰器模式实现只执行一次的功能
def run_once_per_arg(func):
    """
    装饰器：确保被装饰的每个不同参数组合只执行一次
    """
    cache = set()
    def wrapper(*args, **kwargs):
        # 创建参数的唯一标识
        key = (args, frozenset(kwargs.items()))
        if key not in cache:
            cache.add(key)
            return func(*args, **kwargs)
    return wrapper

# 应用装饰器
@run_once_per_arg
def print_once(text):
    """
    打印指定文本的函数（通过装饰器确保只执行一次）
    """
    print(text)

for i in range(10):
    print_once("这是第一条消息")
    print_once("这是第二条消息")