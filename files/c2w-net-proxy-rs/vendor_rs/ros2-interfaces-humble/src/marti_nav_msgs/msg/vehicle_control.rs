use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VehicleControl {
    pub header: crate::std_msgs::msg::Header,
    pub engine: i32,
    pub gear: i32,
    pub steering: f64,
    pub throttle: f64,
    pub brake: f64,
    pub steering_position: i16,
    pub gb_position: i16,
}

impl Default for VehicleControl {
    fn default() -> Self {
        VehicleControl {
            header: crate::std_msgs::msg::Header::default(),
            engine: 0,
            gear: 0,
            steering: 0.0,
            throttle: 0.0,
            brake: 0.0,
            steering_position: 0,
            gb_position: 0,
        }
    }
}

impl ros2_client::Message for VehicleControl {}
