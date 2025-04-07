use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NestedUint8ArrayTestServiceRequest {
    pub input: crate::rclpy_message_converter_msgs::msg::NestedUint8ArrayTestMessage,
}

impl Default for NestedUint8ArrayTestServiceRequest {
    fn default() -> Self {
        NestedUint8ArrayTestServiceRequest {
            input: crate::rclpy_message_converter_msgs::msg::NestedUint8ArrayTestMessage::default(),
        }
    }
}

impl ros2_client::Message for NestedUint8ArrayTestServiceRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NestedUint8ArrayTestServiceResponse {
    pub output: crate::rclpy_message_converter_msgs::msg::NestedUint8ArrayTestMessage,
}

impl Default for NestedUint8ArrayTestServiceResponse {
    fn default() -> Self {
        NestedUint8ArrayTestServiceResponse {
            output: crate::rclpy_message_converter_msgs::msg::NestedUint8ArrayTestMessage::default(),
        }
    }
}

impl ros2_client::Message for NestedUint8ArrayTestServiceResponse {}


pub struct NestedUint8ArrayTestService;
impl ros2_client::Service for NestedUint8ArrayTestService {
    type Request = NestedUint8ArrayTestServiceRequest;
    type Response = NestedUint8ArrayTestServiceResponse;

    fn request_type_name(&self) -> &str { "NestedUint8ArrayTestServiceRequest" }
    fn response_type_name(&self) -> &str { "NestedUint8ArrayTestServiceResponse" }
}
