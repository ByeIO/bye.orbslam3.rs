use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SteeringOffset {
    pub header: crate::std_msgs::msg::Header,
    pub steering_wheel_angle: f32,
    pub steering_wheel_angle_raw: f32,
    pub steering_wheel_angle_offset: f32,
    pub offset_type: u8,
}

impl SteeringOffset {
    pub const OFFSET_UNKNOWN: u8 = 0;
    pub const OFFSET_RELATAIVE: u8 = 1;
    pub const OFFSET_ABSOLUTE: u8 = 2;
}

impl Default for SteeringOffset {
    fn default() -> Self {
        SteeringOffset {
            header: crate::std_msgs::msg::Header::default(),
            steering_wheel_angle: 0.0,
            steering_wheel_angle_raw: 0.0,
            steering_wheel_angle_offset: 0.0,
            offset_type: 0,
        }
    }
}

impl ros2_client::Message for SteeringOffset {}
