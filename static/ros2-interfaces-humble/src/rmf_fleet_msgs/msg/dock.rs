use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Dock {
    pub fleet_name: ::std::string::String,
    pub params: Vec<crate::rmf_fleet_msgs::msg::DockParameter>,
}

impl Default for Dock {
    fn default() -> Self {
        Dock {
            fleet_name: ::std::string::String::new(),
            params: Vec::new(),
        }
    }
}

impl ros2_client::Message for Dock {}
