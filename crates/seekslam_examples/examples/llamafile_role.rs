#![allow(unused)]

//! 测试角色扮演及function calling.

// openai的api接口
use mini_openai;

// unix系统调用
use nix::unistd::execv; // 导入 nix 库中的 execv 函数，用于执行外部程序

// 错误处理
use anyhow::{ Result, anyhow };

// 标准库
// 导入 CString，用于将字符串转换为 C 风格的字符串
use std::ffi::CString; 
// 导入 fs 模块，用于文件操作
use std::fs; 
// 导入 Path，用于路径操作
use std::path::Path; 

const PROMPT : &str = r#"
我是长官，你是一名自动士兵。你的目标进入一间房屋寻找目标`A`然后调用激光器打击5秒, 然后撤离房间。你需要描述你的碎碎念(每次都不能相同), 例如`照明不足`，`房间好黑`。你需要将心理活动带入任务场景，思考过程必须都与任务相关。

以下是你可能遇到的状况:
->现在你进入了房间门口即将起飞。
->现在你在空中侦察房间情况,
->现在你发现前面有水泥柱子, 需要绕过去。
->现在你发现了一个门洞，需要直接穿过去，小心门洞狭窄！
->现在你发现了目标`A`，但是距离超过5米，先不要打开激光。
->现在你离目标`A`距离5米内，处于激光器有效攻击范围内，对准目标`A`打开激光。function laser on;
->5秒时间到，认为目标`A`已经被击杀，后面如果再遇到目标`A`相似的都不要理会，现在转身撤离。
->现在原路返回。
->现在找停机坪`H`标志降落。

从现在开始进入任务场景，你不要思考与任务无关的内容而且思考需要尽量简短，听从指挥。
"#;

const STAGE1_PROMPT : &str = r#"现在你进入了房间门口即将起飞."#;

const STAGE2_PROMPT : &str = r#"现在你在悬停在空中侦察房间情况."#;

const STAGE3_PROMPT : &str = r#"你发现室内有两个房间,当前在第一个房间."#;

fn main()->Result<()>{
    // 记忆是否处理了目标
    let is_target_killed = false;

    // 指定要运行的文件路径
    let file_path = "../../assets/Qwen2.5-0.5B-Instruct-Q4_K_M.llamafile";

    // 检查文件是否存在
    if !Path::new(file_path).exists() {
        println!("文件不存在: {}", file_path);
        return Ok(());
    }

    // 将文件路径转换为 C 风格的字符串
    let c_file_path = CString::new(file_path.as_bytes()).expect("CString::new failed");

    // 构造参数列表，这里假设文件是一个可执行文件，且不需要额外参数
    // 参数列表，第一个参数是文件路径本身
    let args = vec![c_file_path.clone()]; 

    // 使用 execv 执行文件
    match execv(&c_file_path, &args) {
        Ok(_) => println!("文件运行成功"),
        Err(err) => println!("运行文件失败: {:?}", err),
    }

    // 客户端
    let client = mini_openai::Client::new_without_environment("http://127.0.0.1:8080".to_string(), Some(PROMPT.to_string()))?;

    // 创建连接
    let mut request = mini_openai::ChatCompletions::default();

    // 添加到对话历史
    request.messages.push(mini_openai::Message {
        content: STAGE1_PROMPT.to_string(),
        role: mini_openai::ROLE_USER.to_string(),
    });

    // 发送请求
    let response = client.chat_completions(&request)?;

    // 打印结果
    println!("{}", response.choices[0].message.content);

    Ok(())

}