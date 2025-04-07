use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FluidPressure {
    pub header: crate::std_msgs::msg::Header,
    pub fluid_pressure: f64,
    pub variance: f64,
}

impl Default for FluidPressure {
    fn default() -> Self {
        FluidPressure {
            header: crate::std_msgs::msg::Header::default(),
            fluid_pressure: 0.0,
            variance: 0.0,
        }
    }
}

impl ros2_client::Message for FluidPressure {}
