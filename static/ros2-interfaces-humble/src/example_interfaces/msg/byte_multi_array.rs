use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ByteMultiArray {
    pub layout: crate::example_interfaces::msg::MultiArrayLayout,
    pub data: Vec<u8>,
}

impl Default for ByteMultiArray {
    fn default() -> Self {
        ByteMultiArray {
            layout: crate::example_interfaces::msg::MultiArrayLayout::default(),
            data: Vec::new(),
        }
    }
}

impl ros2_client::Message for ByteMultiArray {}
