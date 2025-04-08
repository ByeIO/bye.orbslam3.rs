use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HazardStatusStamped {
    pub stamp: crate::builtin_interfaces::msg::Time,
    pub status: crate::autoware_system_msgs::msg::HazardStatus,
}

impl Default for HazardStatusStamped {
    fn default() -> Self {
        HazardStatusStamped {
            stamp: crate::builtin_interfaces::msg::Time::default(),
            status: crate::autoware_system_msgs::msg::HazardStatus::default(),
        }
    }
}

impl ros2_client::Message for HazardStatusStamped {}
