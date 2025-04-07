use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Status {
    pub header: crate::std_msgs::msg::Header,
    pub id: ::std::string::String,
    pub instance_id: ::std::string::String,
    pub active: bool,
    pub heartbeat_timeout: f32,
    pub heartbeat_period: f32,
}

impl Default for Status {
    fn default() -> Self {
        Status {
            header: crate::std_msgs::msg::Header::default(),
            id: ::std::string::String::new(),
            instance_id: ::std::string::String::new(),
            active: false,
            heartbeat_timeout: 0.0,
            heartbeat_period: 0.0,
        }
    }
}

impl ros2_client::Message for Status {}
