use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParameterDescriptor {
    pub name: ::std::string::String,
    #[serde(rename = "type")]    pub type_: u8,
    pub description: ::std::string::String,
    pub additional_constraints: ::std::string::String,
    pub read_only: bool, // default: false
    pub dynamic_typing: bool, // default: false
    pub floating_point_range: Vec<crate::rcl_interfaces::msg::FloatingPointRange>,
    pub integer_range: Vec<crate::rcl_interfaces::msg::IntegerRange>,
}

impl Default for ParameterDescriptor {
    fn default() -> Self {
        ParameterDescriptor {
            name: ::std::string::String::new(),
            type_: 0,
            description: ::std::string::String::new(),
            additional_constraints: ::std::string::String::new(),
            read_only: false,
            dynamic_typing: false,
            floating_point_range: Vec::new(),
            integer_range: Vec::new(),
        }
    }
}

impl ros2_client::Message for ParameterDescriptor {}
