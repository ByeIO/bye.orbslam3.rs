use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParameterEventDescriptors {
    pub new_parameters: Vec<crate::rcl_interfaces::msg::ParameterDescriptor>,
    pub changed_parameters: Vec<crate::rcl_interfaces::msg::ParameterDescriptor>,
    pub deleted_parameters: Vec<crate::rcl_interfaces::msg::ParameterDescriptor>,
}

impl Default for ParameterEventDescriptors {
    fn default() -> Self {
        ParameterEventDescriptors {
            new_parameters: Vec::new(),
            changed_parameters: Vec::new(),
            deleted_parameters: Vec::new(),
        }
    }
}

impl ros2_client::Message for ParameterEventDescriptors {}
