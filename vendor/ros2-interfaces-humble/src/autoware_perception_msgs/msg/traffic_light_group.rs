use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrafficLightGroup {
    pub traffic_light_group_id: i64,
    pub elements: Vec<crate::autoware_perception_msgs::msg::TrafficLightElement>,
}

impl Default for TrafficLightGroup {
    fn default() -> Self {
        TrafficLightGroup {
            traffic_light_group_id: 0,
            elements: Vec::new(),
        }
    }
}

impl ros2_client::Message for TrafficLightGroup {}
