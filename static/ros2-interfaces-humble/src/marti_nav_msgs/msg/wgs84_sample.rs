use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Wgs84Sample {
    pub header: crate::std_msgs::msg::Header,
    pub odom: crate::geometry_msgs::msg::Point,
    pub wgs84: crate::geometry_msgs::msg::Point,
    pub wgs84_covariance: [f64; 9],
}

impl Default for Wgs84Sample {
    fn default() -> Self {
        Wgs84Sample {
            header: crate::std_msgs::msg::Header::default(),
            odom: crate::geometry_msgs::msg::Point::default(),
            wgs84: crate::geometry_msgs::msg::Point::default(),
            wgs84_covariance: [0.0; 9],
        }
    }
}

impl ros2_client::Message for Wgs84Sample {}
