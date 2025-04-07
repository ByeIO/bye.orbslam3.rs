use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrafficSignalArray {
    pub stamp: crate::builtin_interfaces::msg::Time,
    pub signals: Vec<crate::autoware_perception_msgs::msg::TrafficSignal>,
}

impl Default for TrafficSignalArray {
    fn default() -> Self {
        TrafficSignalArray {
            stamp: crate::builtin_interfaces::msg::Time::default(),
            signals: Vec::new(),
        }
    }
}

impl ros2_client::Message for TrafficSignalArray {}
