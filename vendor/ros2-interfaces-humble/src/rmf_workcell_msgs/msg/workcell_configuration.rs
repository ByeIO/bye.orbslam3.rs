use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkcellConfiguration {
    pub time: crate::builtin_interfaces::msg::Time,
    pub guid: ::std::string::String,
    #[serde(rename = "type")]    pub type_: ::std::string::String,
    pub assets: Vec<crate::rmf_workcell_msgs::msg::Asset>,
    pub traits: Vec<crate::rmf_workcell_msgs::msg::Trait>,
}

impl Default for WorkcellConfiguration {
    fn default() -> Self {
        WorkcellConfiguration {
            time: crate::builtin_interfaces::msg::Time::default(),
            guid: ::std::string::String::new(),
            type_: ::std::string::String::new(),
            assets: Vec::new(),
            traits: Vec::new(),
        }
    }
}

impl ros2_client::Message for WorkcellConfiguration {}
