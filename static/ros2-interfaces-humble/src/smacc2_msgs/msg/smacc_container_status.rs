use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SmaccContainerStatus {
    pub header: crate::std_msgs::msg::Header,
    pub path: ::std::string::String,
    pub initial_states: Vec<::std::string::String>,
    pub active_states: Vec<::std::string::String>,
    pub local_data: ::std::string::String,
    pub info: ::std::string::String,
}

impl Default for SmaccContainerStatus {
    fn default() -> Self {
        SmaccContainerStatus {
            header: crate::std_msgs::msg::Header::default(),
            path: ::std::string::String::new(),
            initial_states: Vec::new(),
            active_states: Vec::new(),
            local_data: ::std::string::String::new(),
            info: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SmaccContainerStatus {}
