use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[serde_as]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarkerWithCovariance {
    pub marker: crate::marker_msgs::msg::Marker,
    #[serde_as(as = "[_; 36]")]
    pub covariance: [f64; 36],
}

impl Default for MarkerWithCovariance {
    fn default() -> Self {
        MarkerWithCovariance {
            marker: crate::marker_msgs::msg::Marker::default(),
            covariance: [0.0; 36],
        }
    }
}

impl ros2_client::Message for MarkerWithCovariance {}
