use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrafficSignal {
    pub traffic_signal_id: i64,
    pub elements: Vec<crate::autoware_perception_msgs::msg::TrafficSignalElement>,
}

impl Default for TrafficSignal {
    fn default() -> Self {
        TrafficSignal {
            traffic_signal_id: 0,
            elements: Vec::new(),
        }
    }
}

impl ros2_client::Message for TrafficSignal {}
