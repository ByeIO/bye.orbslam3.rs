use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RadarTrack {
    pub uuid: crate::unique_identifier_msgs::msg::UUID,
    pub position: crate::geometry_msgs::msg::Point,
    pub velocity: crate::geometry_msgs::msg::Vector3,
    pub acceleration: crate::geometry_msgs::msg::Vector3,
    pub size: crate::geometry_msgs::msg::Vector3,
    pub classification: u16,
    pub position_covariance: [f32; 6],
    pub velocity_covariance: [f32; 6],
    pub acceleration_covariance: [f32; 6],
    pub size_covariance: [f32; 6],
}

impl RadarTrack {
    pub const NO_CLASSIFICATION: u16 = 0;
    pub const STATIC: u16 = 1;
    pub const DYNAMIC: u16 = 2;
}

impl Default for RadarTrack {
    fn default() -> Self {
        RadarTrack {
            uuid: crate::unique_identifier_msgs::msg::UUID::default(),
            position: crate::geometry_msgs::msg::Point::default(),
            velocity: crate::geometry_msgs::msg::Vector3::default(),
            acceleration: crate::geometry_msgs::msg::Vector3::default(),
            size: crate::geometry_msgs::msg::Vector3::default(),
            classification: 0,
            position_covariance: [0.0; 6],
            velocity_covariance: [0.0; 6],
            acceleration_covariance: [0.0; 6],
            size_covariance: [0.0; 6],
        }
    }
}

impl ros2_client::Message for RadarTrack {}
