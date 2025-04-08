use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IMUInfo {
    pub header: crate::std_msgs::msg::Header,
    pub data: [f64; 12],
    pub noise_variances: [f64; 3],
    pub bias_variances: [f64; 3],
}

impl Default for IMUInfo {
    fn default() -> Self {
        IMUInfo {
            header: crate::std_msgs::msg::Header::default(),
            data: [0.0; 12],
            noise_variances: [0.0; 3],
            bias_variances: [0.0; 3],
        }
    }
}

impl ros2_client::Message for IMUInfo {}
