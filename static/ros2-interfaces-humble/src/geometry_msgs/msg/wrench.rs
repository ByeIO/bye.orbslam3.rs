use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Wrench {
    pub force: crate::geometry_msgs::msg::Vector3,
    pub torque: crate::geometry_msgs::msg::Vector3,
}

impl Default for Wrench {
    fn default() -> Self {
        Wrench {
            force: crate::geometry_msgs::msg::Vector3::default(),
            torque: crate::geometry_msgs::msg::Vector3::default(),
        }
    }
}

impl ros2_client::Message for Wrench {}
