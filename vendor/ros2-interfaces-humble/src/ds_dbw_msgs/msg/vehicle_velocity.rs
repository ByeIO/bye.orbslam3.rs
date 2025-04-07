use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VehicleVelocity {
    pub header: crate::std_msgs::msg::Header,
    pub vehicle_velocity_brake: f32,
    pub vehicle_velocity_propulsion: f32,
    pub dir_src: u8,
}

impl VehicleVelocity {
    pub const DIR_NONE: u8 = 0;
    pub const DIR_PRNDL: u8 = 1;
    pub const DIR_SENSOR: u8 = 2;
}

impl Default for VehicleVelocity {
    fn default() -> Self {
        VehicleVelocity {
            header: crate::std_msgs::msg::Header::default(),
            vehicle_velocity_brake: 0.0,
            vehicle_velocity_propulsion: 0.0,
            dir_src: 0,
        }
    }
}

impl ros2_client::Message for VehicleVelocity {}
