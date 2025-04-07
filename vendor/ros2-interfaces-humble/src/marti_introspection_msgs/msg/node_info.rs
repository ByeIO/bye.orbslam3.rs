use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeInfo {
    pub name: ::std::string::String,
    pub location: ::std::string::String,
    pub nodelet_manager: ::std::string::String,
    pub description: ::std::string::String,
    pub topics: Vec<crate::marti_introspection_msgs::msg::TopicInfo>,
    pub parameters: Vec<crate::marti_introspection_msgs::msg::ParamInfo>,
    pub services: Vec<crate::marti_introspection_msgs::msg::ServiceInfo>,
}

impl Default for NodeInfo {
    fn default() -> Self {
        NodeInfo {
            name: ::std::string::String::new(),
            location: ::std::string::String::new(),
            nodelet_manager: ::std::string::String::new(),
            description: ::std::string::String::new(),
            topics: Vec::new(),
            parameters: Vec::new(),
            services: Vec::new(),
        }
    }
}

impl ros2_client::Message for NodeInfo {}
