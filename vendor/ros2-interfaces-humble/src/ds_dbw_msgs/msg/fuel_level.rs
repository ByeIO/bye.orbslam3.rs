use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FuelLevel {
    pub header: crate::std_msgs::msg::Header,
    pub fuel_level: f32,
    pub fuel_range: f32,
    pub odometer: f32,
}

impl Default for FuelLevel {
    fn default() -> Self {
        FuelLevel {
            header: crate::std_msgs::msg::Header::default(),
            fuel_level: 0.0,
            fuel_range: 0.0,
            odometer: 0.0,
        }
    }
}

impl ros2_client::Message for FuelLevel {}
