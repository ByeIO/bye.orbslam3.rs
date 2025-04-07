use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarkerWithCovarianceArray {
    pub header: crate::std_msgs::msg::Header,
    pub markers: Vec<crate::marker_msgs::msg::MarkerWithCovariance>,
}

impl Default for MarkerWithCovarianceArray {
    fn default() -> Self {
        MarkerWithCovarianceArray {
            header: crate::std_msgs::msg::Header::default(),
            markers: Vec::new(),
        }
    }
}

impl ros2_client::Message for MarkerWithCovarianceArray {}
