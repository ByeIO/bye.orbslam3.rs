use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EgoVehicleData {
    pub velocity: crate::geometry_msgs::msg::TwistWithCovariance,
    pub acceleration: crate::geometry_msgs::msg::Accel,
}

impl Default for EgoVehicleData {
    fn default() -> Self {
        EgoVehicleData {
            velocity: crate::geometry_msgs::msg::TwistWithCovariance::default(),
            acceleration: crate::geometry_msgs::msg::Accel::default(),
        }
    }
}

impl ros2_client::Message for EgoVehicleData {}
