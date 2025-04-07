use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BehaviorRequest {
    pub behavior_name: ::std::string::String,
    pub autonomy_level: u8,
    pub arg_keys: Vec<::std::string::String>,
    pub arg_values: Vec<::std::string::String>,
    pub structure: Vec<crate::flexbe_msgs::msg::Container>,
}

impl Default for BehaviorRequest {
    fn default() -> Self {
        BehaviorRequest {
            behavior_name: ::std::string::String::new(),
            autonomy_level: 0,
            arg_keys: Vec::new(),
            arg_values: Vec::new(),
            structure: Vec::new(),
        }
    }
}

impl ros2_client::Message for BehaviorRequest {}
