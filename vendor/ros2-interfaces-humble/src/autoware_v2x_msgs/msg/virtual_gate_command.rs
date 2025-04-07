use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VirtualGateCommand {
    pub stamp: crate::builtin_interfaces::msg::Time,
    pub areas: Vec<crate::autoware_v2x_msgs::msg::VirtualGateAreaCommand>,
}

impl Default for VirtualGateCommand {
    fn default() -> Self {
        VirtualGateCommand {
            stamp: crate::builtin_interfaces::msg::Time::default(),
            areas: Vec::new(),
        }
    }
}

impl ros2_client::Message for VirtualGateCommand {}
