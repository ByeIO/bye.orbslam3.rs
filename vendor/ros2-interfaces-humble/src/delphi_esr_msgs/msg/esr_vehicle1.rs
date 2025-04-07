use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EsrVehicle1 {
    pub header: crate::std_msgs::msg::Header,
    pub vehicle_speed: f32,
    pub vehicle_speed_direction: bool,
    pub yaw_rate: f32,
    pub yaw_rate_validity: bool,
    pub steering_angle_rate_sign: bool,
    pub radius_curvature: i16,
    pub steering_angle_validity: bool,
    pub steering_angle_sign: bool,
    pub steering_angle: u16,
    pub steering_angle_rate: u16,
}

impl Default for EsrVehicle1 {
    fn default() -> Self {
        EsrVehicle1 {
            header: crate::std_msgs::msg::Header::default(),
            vehicle_speed: 0.0,
            vehicle_speed_direction: false,
            yaw_rate: 0.0,
            yaw_rate_validity: false,
            steering_angle_rate_sign: false,
            radius_curvature: 0,
            steering_angle_validity: false,
            steering_angle_sign: false,
            steering_angle: 0,
            steering_angle_rate: 0,
        }
    }
}

impl ros2_client::Message for EsrVehicle1 {}
