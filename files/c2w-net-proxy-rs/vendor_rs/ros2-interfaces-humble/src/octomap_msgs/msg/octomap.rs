use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Octomap {
    pub header: crate::std_msgs::msg::Header,
    pub binary: bool,
    pub id: ::std::string::String,
    pub resolution: f64,
    pub data: Vec<i8>,
}

impl Default for Octomap {
    fn default() -> Self {
        Octomap {
            header: crate::std_msgs::msg::Header::default(),
            binary: false,
            id: ::std::string::String::new(),
            resolution: 0.0,
            data: Vec::new(),
        }
    }
}

impl ros2_client::Message for Octomap {}
