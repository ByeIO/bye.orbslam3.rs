use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrackedObject {
    pub header: crate::std_msgs::msg::Header,
    pub id: u16,
    pub pose: crate::geometry_msgs::msg::PoseWithCovariance,
    pub velocity: crate::geometry_msgs::msg::TwistWithCovariance,
    pub linear_acceleration: crate::geometry_msgs::msg::Vector3,
    pub linear_acceleration_covariance: [f64; 9],
    pub polygon: Vec<crate::geometry_msgs::msg::Point>,
    pub length: f32,
    pub length_quality: f32,
    pub width: f32,
    pub width_quality: f32,
    pub classification: u8,
    pub classification_quality: f32,
    pub existence_probability: f32,
    pub age_duration: crate::builtin_interfaces::msg::Duration,
    pub prediction_duration: crate::builtin_interfaces::msg::Duration,
    pub active: bool,
}

impl TrackedObject {
    pub const VEHICLE: u8 = 0;
    pub const PEDESTRIAN: u8 = 1;
    pub const UNKNOWN: u8 = 255;
}

impl Default for TrackedObject {
    fn default() -> Self {
        TrackedObject {
            header: crate::std_msgs::msg::Header::default(),
            id: 0,
            pose: crate::geometry_msgs::msg::PoseWithCovariance::default(),
            velocity: crate::geometry_msgs::msg::TwistWithCovariance::default(),
            linear_acceleration: crate::geometry_msgs::msg::Vector3::default(),
            linear_acceleration_covariance: [0.0; 9],
            polygon: Vec::new(),
            length: 0.0,
            length_quality: 0.0,
            width: 0.0,
            width_quality: 0.0,
            classification: 0,
            classification_quality: 0.0,
            existence_probability: 0.0,
            age_duration: crate::builtin_interfaces::msg::Duration::default(),
            prediction_duration: crate::builtin_interfaces::msg::Duration::default(),
            active: false,
        }
    }
}

impl ros2_client::Message for TrackedObject {}
