use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NestedUint8ArrayTestMessage {
    pub arrays: Vec<crate::rclpy_message_converter_msgs::msg::Uint8ArrayTestMessage>,
}

impl Default for NestedUint8ArrayTestMessage {
    fn default() -> Self {
        NestedUint8ArrayTestMessage {
            arrays: Vec::new(),
        }
    }
}

impl ros2_client::Message for NestedUint8ArrayTestMessage {}
