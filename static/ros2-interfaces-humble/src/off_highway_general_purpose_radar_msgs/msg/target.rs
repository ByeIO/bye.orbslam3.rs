use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Target {
    pub header: crate::std_msgs::msg::Header,
    pub a: crate::off_highway_general_purpose_radar_msgs::msg::TargetA,
    pub b: crate::off_highway_general_purpose_radar_msgs::msg::TargetB,
}

impl Default for Target {
    fn default() -> Self {
        Target {
            header: crate::std_msgs::msg::Header::default(),
            a: crate::off_highway_general_purpose_radar_msgs::msg::TargetA::default(),
            b: crate::off_highway_general_purpose_radar_msgs::msg::TargetB::default(),
        }
    }
}

impl ros2_client::Message for Target {}
