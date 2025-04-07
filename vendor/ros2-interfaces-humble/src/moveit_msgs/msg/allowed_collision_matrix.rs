use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AllowedCollisionMatrix {
    pub entry_names: Vec<::std::string::String>,
    pub entry_values: Vec<crate::moveit_msgs::msg::AllowedCollisionEntry>,
    pub default_entry_names: Vec<::std::string::String>,
    pub default_entry_values: Vec<bool>,
}

impl Default for AllowedCollisionMatrix {
    fn default() -> Self {
        AllowedCollisionMatrix {
            entry_names: Vec::new(),
            entry_values: Vec::new(),
            default_entry_names: Vec::new(),
            default_entry_values: Vec::new(),
        }
    }
}

impl ros2_client::Message for AllowedCollisionMatrix {}
