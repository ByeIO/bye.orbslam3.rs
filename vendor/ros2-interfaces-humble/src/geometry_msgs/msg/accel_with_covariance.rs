use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[serde_as]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccelWithCovariance {
    pub accel: crate::geometry_msgs::msg::Accel,
    #[serde_as(as = "[_; 36]")]
    pub covariance: [f64; 36],
}

impl Default for AccelWithCovariance {
    fn default() -> Self {
        AccelWithCovariance {
            accel: crate::geometry_msgs::msg::Accel::default(),
            covariance: [0.0; 36],
        }
    }
}

impl ros2_client::Message for AccelWithCovariance {}
