use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Gaze {
    pub header: crate::std_msgs::msg::Header,
    pub sender: ::std::string::String,
    pub receiver: ::std::string::String,
}

impl Default for Gaze {
    fn default() -> Self {
        Gaze {
            header: crate::std_msgs::msg::Header::default(),
            sender: ::std::string::String::new(),
            receiver: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for Gaze {}
