use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DockSummary {
    pub docks: Vec<crate::rmf_fleet_msgs::msg::Dock>,
}

impl Default for DockSummary {
    fn default() -> Self {
        DockSummary {
            docks: Vec::new(),
        }
    }
}

impl ros2_client::Message for DockSummary {}
