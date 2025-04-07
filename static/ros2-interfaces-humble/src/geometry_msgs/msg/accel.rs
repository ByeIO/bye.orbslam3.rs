use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Accel {
    pub linear: crate::geometry_msgs::msg::Vector3,
    pub angular: crate::geometry_msgs::msg::Vector3,
}

impl Default for Accel {
    fn default() -> Self {
        Accel {
            linear: crate::geometry_msgs::msg::Vector3::default(),
            angular: crate::geometry_msgs::msg::Vector3::default(),
        }
    }
}

impl ros2_client::Message for Accel {}
