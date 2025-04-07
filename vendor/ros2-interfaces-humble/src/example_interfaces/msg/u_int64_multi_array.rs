use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UInt64MultiArray {
    pub layout: crate::example_interfaces::msg::MultiArrayLayout,
    pub data: Vec<u64>,
}

impl Default for UInt64MultiArray {
    fn default() -> Self {
        UInt64MultiArray {
            layout: crate::example_interfaces::msg::MultiArrayLayout::default(),
            data: Vec::new(),
        }
    }
}

impl ros2_client::Message for UInt64MultiArray {}
