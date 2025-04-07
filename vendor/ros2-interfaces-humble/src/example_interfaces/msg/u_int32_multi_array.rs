use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UInt32MultiArray {
    pub layout: crate::example_interfaces::msg::MultiArrayLayout,
    pub data: Vec<u32>,
}

impl Default for UInt32MultiArray {
    fn default() -> Self {
        UInt32MultiArray {
            layout: crate::example_interfaces::msg::MultiArrayLayout::default(),
            data: Vec::new(),
        }
    }
}

impl ros2_client::Message for UInt32MultiArray {}
