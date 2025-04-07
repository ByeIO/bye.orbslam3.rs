#![allow(unreachable_patterns)]

//! mcp消息通信客户端

use rust_mcp_schema::schema_utils::*;
use rust_mcp_schema::*;
use std::str::FromStr;

type AppError = JsonrpcErrorError;

const SAMPLE_PAYLOAD: &str = r#"
{
    "id": 0,
    "jsonrpc": "2.0",
    "result": {
        "protocolVersion": "2024-11-05",
        "capabilities": {
            "prompts": {},
            "resources": {
                "subscribe": true
            },
            "tools": {},
            "logging": {}
        },
        "serverInfo": {
            "name": "example-servers/everything",
            "version": "1.0.0"
        }
    }
}
"#;

fn main() -> std::result::Result<(), AppError> {
    handle_message(SAMPLE_PAYLOAD)?;
    Ok(())
}

/// 将 JSON-RPC 消息反序列化为相应的 MCP 类型，并使用 dbg!() 宏打印。
fn handle_message(message_payload: &str) -> std::result::Result<(), AppError> {
    // 将消息反序列化为 ServerMessage。
    // ServerMessage 表示由 MCP 服务器发送并由 MCP 客户端接收的消息。
    let mcp_message = ServerMessage::from_str(message_payload)?;

    match mcp_message {
        // 检查消息是否为请求
        ServerMessage::Request(server_message) => match server_message.request {
            // 检查是否为标准的 ServerRequest（非 CustomRequest）
            RequestFromServer::ServerRequest(server_request) => {
                // 处理不同的 ServerNotifications
                match server_request {
                    ServerRequest::PingRequest(ping_request) => {
                        dbg!(ping_request);
                    }
                    ServerRequest::CreateMessageRequest(create_message_request) => {
                        dbg!(create_message_request);
                    }
                    ServerRequest::ListRootsRequest(list_roots_request) => {
                        dbg!(list_roots_request);
                    }
                }
            }
            // 检查是否为 CustomRequest；该值可以反序列化为自定义类型。
            RequestFromServer::CustomRequest(value) => {
                dbg!(value);
            }
        },
        // 检查消息是否为通知
        ServerMessage::Notification(server_message) => match server_message.notification {
            // 检查是否为标准的 ServerNotification（非 CustomNotification）
            NotificationFromServer::ServerNotification(server_notification) => {
                // 处理不同的 ServerNotifications
                match server_notification {
                    ServerNotification::CancelledNotification(cancelled_notification) => {
                        dbg!(cancelled_notification);
                    }
                    ServerNotification::ProgressNotification(progress_notification) => {
                        dbg!(progress_notification);
                    }
                    ServerNotification::ResourceListChangedNotification(resource_list_changed_notification) => {
                        dbg!(resource_list_changed_notification);
                    }
                    ServerNotification::ResourceUpdatedNotification(resource_updated_notification) => {
                        dbg!(resource_updated_notification);
                    }
                    ServerNotification::PromptListChangedNotification(prompt_list_changed_notification) => {
                        dbg!(prompt_list_changed_notification);
                    }
                    ServerNotification::ToolListChangedNotification(tool_list_changed_notification) => {
                        dbg!(tool_list_changed_notification);
                    }
                    ServerNotification::LoggingMessageNotification(logging_message_notification) => {
                        dbg!(logging_message_notification);
                    }
                }
            }
            // 检查是否为 CustomNotification；该值可以反序列化为自定义类型。
            NotificationFromServer::CustomNotification(value) => {
                dbg!(value);
            }
        },
        // 检查消息是否为响应
        ServerMessage::Response(server_message) => match server_message.result {
            // 检查是否为标准的 ServerResult（非 CustomResult）
            ResultFromServer::ServerResult(server_result) => match server_result {
                ServerResult::Result(result) => {
                    dbg!(result);
                }
                ServerResult::InitializeResult(initialize_result) => {
                    dbg!(initialize_result);
                }
                ServerResult::ListResourcesResult(list_resources_result) => {
                    dbg!(list_resources_result);
                }
                #[cfg(feature = "2024_11_05")]
                ServerResult::ListResourceTemplatesResult(list_resource_templates_result) => {
                    dbg!(list_resource_templates_result);
                }
                ServerResult::ReadResourceResult(read_resource_result) => {
                    dbg!(read_resource_result);
                }
                ServerResult::ListPromptsResult(list_prompts_result) => {
                    dbg!(list_prompts_result);
                }
                ServerResult::GetPromptResult(get_prompt_result) => {
                    dbg!(get_prompt_result);
                }
                ServerResult::ListToolsResult(list_tools_result) => {
                    dbg!(list_tools_result);
                }
                ServerResult::CallToolResult(call_tool_result) => {
                    dbg!(call_tool_result);
                }
                ServerResult::CompleteResult(complete_result) => {
                    dbg!(complete_result);
                },
                // 其余情况
                _ => {
                    // do nothing
                }
            },
            // 检查是否为 CustomResult；该值可以反序列化为自定义类型。
            ResultFromServer::CustomResult(value) => {
                dbg!(value);
            },
            // 其余情况
            _ => {
                // do nothing
            },
        },
        // 检查是否为错误消息
        ServerMessage::Error(server_message) => {
            dbg!(server_message);
        }, 
        // 其余情况
        _ => {
            // do nothing
        },
    }
    Ok(())
}
