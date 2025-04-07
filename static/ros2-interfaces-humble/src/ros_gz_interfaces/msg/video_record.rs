use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VideoRecord {
    pub header: crate::std_msgs::msg::Header,
    pub start: bool,
    pub stop: bool,
    pub format: ::std::string::String,
    pub save_filename: ::std::string::String,
}

impl Default for VideoRecord {
    fn default() -> Self {
        VideoRecord {
            header: crate::std_msgs::msg::Header::default(),
            start: false,
            stop: false,
            format: ::std::string::String::new(),
            save_filename: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for VideoRecord {}
