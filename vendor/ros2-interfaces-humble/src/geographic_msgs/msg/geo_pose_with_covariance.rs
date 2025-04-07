use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[serde_as]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeoPoseWithCovariance {
    pub pose: crate::geographic_msgs::msg::GeoPose,
    #[serde_as(as = "[_; 36]")]
    pub covariance: [f64; 36],
}

impl Default for GeoPoseWithCovariance {
    fn default() -> Self {
        GeoPoseWithCovariance {
            pose: crate::geographic_msgs::msg::GeoPose::default(),
            covariance: [0.0; 36],
        }
    }
}

impl ros2_client::Message for GeoPoseWithCovariance {}
