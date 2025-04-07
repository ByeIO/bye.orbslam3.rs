use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Int8MultiArray {
    pub layout: crate::example_interfaces::msg::MultiArrayLayout,
    pub data: Vec<i8>,
}

impl Default for Int8MultiArray {
    fn default() -> Self {
        Int8MultiArray {
            layout: crate::example_interfaces::msg::MultiArrayLayout::default(),
            data: Vec::new(),
        }
    }
}

impl ros2_client::Message for Int8MultiArray {}
