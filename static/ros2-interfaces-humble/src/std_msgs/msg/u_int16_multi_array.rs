use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UInt16MultiArray {
    pub layout: crate::std_msgs::msg::MultiArrayLayout,
    pub data: Vec<u16>,
}

impl Default for UInt16MultiArray {
    fn default() -> Self {
        UInt16MultiArray {
            layout: crate::std_msgs::msg::MultiArrayLayout::default(),
            data: Vec::new(),
        }
    }
}

impl ros2_client::Message for UInt16MultiArray {}
