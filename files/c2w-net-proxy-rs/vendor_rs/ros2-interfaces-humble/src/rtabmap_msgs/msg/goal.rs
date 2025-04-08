use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Goal {
    pub header: crate::std_msgs::msg::Header,
    pub node_id: i32,
    pub node_label: ::std::string::String,
    pub frame_id: ::std::string::String,
}

impl Default for Goal {
    fn default() -> Self {
        Goal {
            header: crate::std_msgs::msg::Header::default(),
            node_id: 0,
            node_label: ::std::string::String::new(),
            frame_id: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for Goal {}
