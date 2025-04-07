use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SensorNoise {
    pub header: crate::std_msgs::msg::Header,
    #[serde(rename = "type")]    pub type_: u8,
    pub mean: f64,
    pub stddev: f64,
    pub bias_mean: f64,
    pub bias_stddev: f64,
    pub precision: f64,
    pub dynamic_bias_stddev: f64,
    pub dynamic_bias_correlation_time: f64,
}

impl SensorNoise {
    pub const NONE: u8 = 0;
    pub const GAUSSIAN: u8 = 2;
    pub const GAUSSIAN_QUANTIZED: u8 = 3;
}

impl Default for SensorNoise {
    fn default() -> Self {
        SensorNoise {
            header: crate::std_msgs::msg::Header::default(),
            type_: 0,
            mean: 0.0,
            stddev: 0.0,
            bias_mean: 0.0,
            bias_stddev: 0.0,
            precision: 0.0,
            dynamic_bias_stddev: 0.0,
            dynamic_bias_correlation_time: 0.0,
        }
    }
}

impl ros2_client::Message for SensorNoise {}
