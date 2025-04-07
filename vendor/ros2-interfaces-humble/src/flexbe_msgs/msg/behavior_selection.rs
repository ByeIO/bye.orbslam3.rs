use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BehaviorSelection {
    pub behavior_key: i32,
    pub behavior_id: i32,
    pub autonomy_level: u8,
    pub arg_keys: Vec<::std::string::String>,
    pub arg_values: Vec<::std::string::String>,
    pub input_keys: Vec<::std::string::String>,
    pub input_values: Vec<::std::string::String>,
    pub modifications: Vec<crate::flexbe_msgs::msg::BehaviorModification>,
}

impl Default for BehaviorSelection {
    fn default() -> Self {
        BehaviorSelection {
            behavior_key: 0,
            behavior_id: 0,
            autonomy_level: 0,
            arg_keys: Vec::new(),
            arg_values: Vec::new(),
            input_keys: Vec::new(),
            input_values: Vec::new(),
            modifications: Vec::new(),
        }
    }
}

impl ros2_client::Message for BehaviorSelection {}
