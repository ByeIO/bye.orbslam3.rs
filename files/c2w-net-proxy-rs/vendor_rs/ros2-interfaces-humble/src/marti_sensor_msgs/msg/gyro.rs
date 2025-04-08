use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Gyro {
    pub header: crate::std_msgs::msg::Header,
    pub angular_rate: f64,
    pub variance: f64,
}

impl Default for Gyro {
    fn default() -> Self {
        Gyro {
            header: crate::std_msgs::msg::Header::default(),
            angular_rate: 0.0,
            variance: 0.0,
        }
    }
}

impl ros2_client::Message for Gyro {}
