use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VehicleOdometry {
    pub header: crate::std_msgs::msg::Header,
    pub velocity: f32,
    pub front_wheel_angle: f32,
    pub rear_wheel_angle: f32,
}

impl Default for VehicleOdometry {
    fn default() -> Self {
        VehicleOdometry {
            header: crate::std_msgs::msg::Header::default(),
            velocity: 0.0,
            front_wheel_angle: 0.0,
            rear_wheel_angle: 0.0,
        }
    }
}

impl ros2_client::Message for VehicleOdometry {}
