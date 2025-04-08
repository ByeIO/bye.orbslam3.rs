use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Compartment {
    pub pose: crate::geometry_msgs::msg::Pose,
    #[serde(rename = "box")]    pub box_: crate::rc_reason_msgs::msg::Box,
}

impl Default for Compartment {
    fn default() -> Self {
        Compartment {
            pose: crate::geometry_msgs::msg::Pose::default(),
            box_: crate::rc_reason_msgs::msg::Box::default(),
        }
    }
}

impl ros2_client::Message for Compartment {}
