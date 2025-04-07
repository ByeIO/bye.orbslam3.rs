use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarkerWithCovarianceStamped {
    pub header: crate::std_msgs::msg::Header,
    pub marker: crate::marker_msgs::msg::MarkerWithCovariance,
}

impl Default for MarkerWithCovarianceStamped {
    fn default() -> Self {
        MarkerWithCovarianceStamped {
            header: crate::std_msgs::msg::Header::default(),
            marker: crate::marker_msgs::msg::MarkerWithCovariance::default(),
        }
    }
}

impl ros2_client::Message for MarkerWithCovarianceStamped {}
