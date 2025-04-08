use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComputeTime {
    pub header: crate::std_msgs::msg::Header,
    pub duration: crate::builtin_interfaces::msg::Duration,
    pub id: ::std::string::String,
    pub parent_id: ::std::string::String,
}

impl Default for ComputeTime {
    fn default() -> Self {
        ComputeTime {
            header: crate::std_msgs::msg::Header::default(),
            duration: crate::builtin_interfaces::msg::Duration::default(),
            id: ::std::string::String::new(),
            parent_id: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ComputeTime {}
