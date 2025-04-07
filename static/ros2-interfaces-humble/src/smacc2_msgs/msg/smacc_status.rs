use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SmaccStatus {
    pub header: crate::std_msgs::msg::Header,
    pub current_states: Vec<::std::string::String>,
    pub global_variable_names: Vec<::std::string::String>,
    pub global_variable_values: Vec<::std::string::String>,
}

impl Default for SmaccStatus {
    fn default() -> Self {
        SmaccStatus {
            header: crate::std_msgs::msg::Header::default(),
            current_states: Vec::new(),
            global_variable_names: Vec::new(),
            global_variable_values: Vec::new(),
        }
    }
}

impl ros2_client::Message for SmaccStatus {}
