use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Parameter {
    pub name: ::std::string::String,
    pub value: crate::rcl_interfaces::msg::ParameterValue,
}

impl Default for Parameter {
    fn default() -> Self {
        Parameter {
            name: ::std::string::String::new(),
            value: crate::rcl_interfaces::msg::ParameterValue::default(),
        }
    }
}

impl ros2_client::Message for Parameter {}
