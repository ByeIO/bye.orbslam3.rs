use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Param {
    pub header: crate::std_msgs::msg::Header,
    pub param_id: ::std::string::String,
    pub value: crate::mavros_msgs::msg::ParamValue,
    pub param_index: u16,
    pub param_count: u16,
}

impl Default for Param {
    fn default() -> Self {
        Param {
            header: crate::std_msgs::msg::Header::default(),
            param_id: ::std::string::String::new(),
            value: crate::mavros_msgs::msg::ParamValue::default(),
            param_index: 0,
            param_count: 0,
        }
    }
}

impl ros2_client::Message for Param {}
