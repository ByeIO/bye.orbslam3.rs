use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AirskinInfo {
    pub header: crate::std_msgs::msg::Header,
    pub ids: Vec<u8>,
    pub names: Vec<::std::string::String>,
    pub min: Vec<u32>,
    pub max: Vec<u32>,
}

impl Default for AirskinInfo {
    fn default() -> Self {
        AirskinInfo {
            header: crate::std_msgs::msg::Header::default(),
            ids: Vec::new(),
            names: Vec::new(),
            min: Vec::new(),
            max: Vec::new(),
        }
    }
}

impl ros2_client::Message for AirskinInfo {}
