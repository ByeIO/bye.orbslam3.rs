use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UInt8MultiArray {
    pub layout: crate::std_msgs::msg::MultiArrayLayout,
    pub data: Vec<u8>,
}

impl Default for UInt8MultiArray {
    fn default() -> Self {
        UInt8MultiArray {
            layout: crate::std_msgs::msg::MultiArrayLayout::default(),
            data: Vec::new(),
        }
    }
}

impl ros2_client::Message for UInt8MultiArray {}
