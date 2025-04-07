use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Command {
    pub header: crate::std_msgs::msg::Header,
    pub startstop: Vec<i32>,
}

impl Default for Command {
    fn default() -> Self {
        Command {
            header: crate::std_msgs::msg::Header::default(),
            startstop: Vec::new(),
        }
    }
}

impl ros2_client::Message for Command {}
