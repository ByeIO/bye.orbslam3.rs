#![allow(unreachable_patterns)]

//! mcp消息通信服务端

use rust_mcp_schema::schema_utils::*;
use rust_mcp_schema::*;
use std::str::FromStr;

type AppError = JsonrpcErrorError;

// 示例消息载荷
const SAMPLE_PAYLOAD: &str = r#"
{
    "id": 0,
    "jsonrpc": "2.0",
    "method": "initialize",
    "params": {
        "protocolVersion": "2024-11-05",
        "capabilities": {
            "sampling": {},
            "roots": {
                "listChanged": true
            }
        },
        "clientInfo": {
            "name": "mcp-inspector",
            "version": "0.1.0"
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
    // 将消息反序列化为 ClientMessage。
    // ClientMessage 表示由 MCP 客户端发送并由 MCP 服务器接收的消息。
    let mcp_message = ClientMessage::from_str(message_payload)?;

    match mcp_message {
        // 检查消息是否为请求
        ClientMessage::Request(client_message) => match client_message.request {
            // 检查是否为标准的 ClientRequest（非 CustomRequest）
            RequestFromClient::ClientRequest(client_request) => match client_request {
                ClientRequest::InitializeRequest(initialize_request) => {
                    dbg!(initialize_request);
                }

                ClientRequest::PingRequest(ping_request) => {
                    dbg!(ping_request);
                }
                ClientRequest::ListResourcesRequest(list_resources_request) => {
                    dbg!(list_resources_request);
                }

                #[cfg(feature = "2024_11_05")]
                ClientRequest::ListResourceTemplatesRequest(list_resource_templates_request) => {
                    dbg!(list_resource_templates_request);
                }

                ClientRequest::ReadResourceRequest(read_resource_request) => {
                    dbg!(read_resource_request);
                }

                ClientRequest::SubscribeRequest(subscribe_request) => {
                    dbg!(subscribe_request);
                }

                ClientRequest::UnsubscribeRequest(unsubscribe_request) => {
                    dbg!(unsubscribe_request);
                }

                ClientRequest::ListPromptsRequest(list_prompts_request) => {
                    dbg!(list_prompts_request);
                }

                ClientRequest::GetPromptRequest(get_prompt_request) => {
                    dbg!(get_prompt_request);
                }

                ClientRequest::ListToolsRequest(list_tools_request) => {
                    dbg!(list_tools_request);
                }

                ClientRequest::CallToolRequest(call_tool_request) => {
                    dbg!(call_tool_request);
                }

                ClientRequest::SetLevelRequest(set_level_request) => {
                    dbg!(set_level_request);
                }

                ClientRequest::CompleteRequest(complete_request) => {
                    dbg!(complete_request);
                },
                _ => {
                    // 待处理
                },
            },

            // 检查是否为 CustomRequest；该值可以反序列化为自定义类型。
            RequestFromClient::CustomRequest(value) => {
                dbg!(value);
            }
        },

        // 检查消息是否为通知
        ClientMessage::Notification(client_message) => match client_message.notification {
            // 检查是否为标准的 ClientNotification（非 CustomNotification）
            NotificationFromClient::ClientNotification(client_notification) => {
                // 处理不同的 ClientNotifications
                match client_notification {
                    ClientNotification::CancelledNotification(cancelled_notification) => {
                        dbg!(cancelled_notification);
                    }

                    ClientNotification::InitializedNotification(initialized_notification) => {
                        dbg!(initialized_notification);
                    }

                    ClientNotification::ProgressNotification(progress_notification) => {
                        dbg!(progress_notification);
                    }

                    ClientNotification::RootsListChangedNotification(progress_notification) => {
                        dbg!(progress_notification);
                    }
                }
            }

            // 检查是否为 CustomNotification；该值可以反序列化为自定义类型。
            NotificationFromClient::CustomNotification(value) => {
                dbg!(value);
            }
        },

        // 检查消息是否为响应
        ClientMessage::Response(client_message) => match client_message.result {
            // 检查是否为标准的 ClientResult（非 CustomResult）
            ResultFromClient::ClientResult(client_result) => match client_result {
                ClientResult::Result(_) => {
                    dbg!(client_result);
                }

                ClientResult::CreateMessageResult(create_message_result) => {
                    dbg!(create_message_result);
                }

                ClientResult::ListRootsResult(list_roots_result) => {
                    dbg!(list_roots_result);
                }
            },

            // 检查是否为 CustomResult；该值可以反序列化为自定义类型。
            ResultFromClient::CustomResult(value) => {
                dbg!(value);
            }
        },

        // 检查是否为错误消息
        ClientMessage::Error(client_error) => {
            dbg!(client_error);
        }
    }
    Ok(())
}
