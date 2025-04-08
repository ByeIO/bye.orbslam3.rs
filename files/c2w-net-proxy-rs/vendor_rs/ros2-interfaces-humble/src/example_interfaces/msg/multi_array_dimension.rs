use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MultiArrayDimension {
    pub label: ::std::string::String,
    pub size: u32,
    pub stride: u32,
}

impl Default for MultiArrayDimension {
    fn default() -> Self {
        MultiArrayDimension {
            label: ::std::string::String::new(),
            size: 0,
            stride: 0,
        }
    }
}

impl ros2_client::Message for MultiArrayDimension {}
