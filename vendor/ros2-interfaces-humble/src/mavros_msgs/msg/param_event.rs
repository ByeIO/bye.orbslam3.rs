use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParamEvent {
    pub header: crate::std_msgs::msg::Header,
    pub param_id: ::std::string::String,
    pub value: crate::rcl_interfaces::msg::ParameterValue,
    pub param_index: u16,
    pub param_count: u16,
}

impl Default for ParamEvent {
    fn default() -> Self {
        ParamEvent {
            header: crate::std_msgs::msg::Header::default(),
            param_id: ::std::string::String::new(),
            value: crate::rcl_interfaces::msg::ParameterValue::default(),
            param_index: 0,
            param_count: 0,
        }
    }
}

impl ros2_client::Message for ParamEvent {}
