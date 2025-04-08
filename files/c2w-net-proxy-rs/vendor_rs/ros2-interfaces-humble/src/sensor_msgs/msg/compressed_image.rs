use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompressedImage {
    pub header: crate::std_msgs::msg::Header,
    pub format: ::std::string::String,
    pub data: Vec<u8>,
}

impl Default for CompressedImage {
    fn default() -> Self {
        CompressedImage {
            header: crate::std_msgs::msg::Header::default(),
            format: ::std::string::String::new(),
            data: Vec::new(),
        }
    }
}

impl ros2_client::Message for CompressedImage {}
