use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Speed {
    pub header: crate::std_msgs::msg::Header,
    pub module_name: ::std::string::String,
    pub speed: f32,
    pub acceleration_limit: f32,
    pub deceleration_limit: f32,
}

impl Default for Speed {
    fn default() -> Self {
        Speed {
            header: crate::std_msgs::msg::Header::default(),
            module_name: ::std::string::String::new(),
            speed: 0.0,
            acceleration_limit: 0.0,
            deceleration_limit: 0.0,
        }
    }
}

impl ros2_client::Message for Speed {}
