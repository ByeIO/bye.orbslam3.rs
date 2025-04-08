use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SmaccContainerStructure {
    pub header: crate::std_msgs::msg::Header,
    pub path: ::std::string::String,
    pub children: Vec<::std::string::String>,
    pub internal_outcomes: Vec<::std::string::String>,
    pub outcomes_from: Vec<::std::string::String>,
    pub outcomes_to: Vec<::std::string::String>,
    pub container_outcomes: Vec<::std::string::String>,
}

impl Default for SmaccContainerStructure {
    fn default() -> Self {
        SmaccContainerStructure {
            header: crate::std_msgs::msg::Header::default(),
            path: ::std::string::String::new(),
            children: Vec::new(),
            internal_outcomes: Vec::new(),
            outcomes_from: Vec::new(),
            outcomes_to: Vec::new(),
            container_outcomes: Vec::new(),
        }
    }
}

impl ros2_client::Message for SmaccContainerStructure {}
