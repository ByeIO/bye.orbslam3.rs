use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttitudeTarget {
    pub header: crate::std_msgs::msg::Header,
    pub type_mask: u8,
    pub orientation: crate::geometry_msgs::msg::Quaternion,
    pub body_rate: crate::geometry_msgs::msg::Vector3,
    pub thrust: f32,
}

impl AttitudeTarget {
    pub const IGNORE_ROLL_RATE: u8 = 1;
    pub const IGNORE_PITCH_RATE: u8 = 2;
    pub const IGNORE_YAW_RATE: u8 = 4;
    pub const IGNORE_THRUST: u8 = 64;
    pub const IGNORE_ATTITUDE: u8 = 128;
}

impl Default for AttitudeTarget {
    fn default() -> Self {
        AttitudeTarget {
            header: crate::std_msgs::msg::Header::default(),
            type_mask: 0,
            orientation: crate::geometry_msgs::msg::Quaternion::default(),
            body_rate: crate::geometry_msgs::msg::Vector3::default(),
            thrust: 0.0,
        }
    }
}

impl ros2_client::Message for AttitudeTarget {}
