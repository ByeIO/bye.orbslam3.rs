use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LaneModels {
    pub header: crate::std_msgs::msg::Header,
    pub left_lane: crate::derived_object_msgs::msg::Lane,
    pub right_lane: crate::derived_object_msgs::msg::Lane,
    pub additional_lanes: Vec<crate::derived_object_msgs::msg::Lane>,
}

impl Default for LaneModels {
    fn default() -> Self {
        LaneModels {
            header: crate::std_msgs::msg::Header::default(),
            left_lane: crate::derived_object_msgs::msg::Lane::default(),
            right_lane: crate::derived_object_msgs::msg::Lane::default(),
            additional_lanes: Vec::new(),
        }
    }
}

impl ros2_client::Message for LaneModels {}
