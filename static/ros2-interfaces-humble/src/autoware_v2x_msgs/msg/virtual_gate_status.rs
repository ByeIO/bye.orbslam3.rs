use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VirtualGateStatus {
    pub areas: Vec<crate::autoware_v2x_msgs::msg::VirtualGateAreaStatus>,
}

impl Default for VirtualGateStatus {
    fn default() -> Self {
        VirtualGateStatus {
            areas: Vec::new(),
        }
    }
}

impl ros2_client::Message for VirtualGateStatus {}
