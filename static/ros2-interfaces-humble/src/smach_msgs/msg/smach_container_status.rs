use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SmachContainerStatus {
    pub header: crate::std_msgs::msg::Header,
    pub path: ::std::string::String,
    pub initial_states: Vec<::std::string::String>,
    pub active_states: Vec<::std::string::String>,
    pub local_data: Vec<u8>,
    pub info: ::std::string::String,
}

impl Default for SmachContainerStatus {
    fn default() -> Self {
        SmachContainerStatus {
            header: crate::std_msgs::msg::Header::default(),
            path: ::std::string::String::new(),
            initial_states: Vec::new(),
            active_states: Vec::new(),
            local_data: Vec::new(),
            info: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SmachContainerStatus {}
