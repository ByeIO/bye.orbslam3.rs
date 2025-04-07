use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[serde_as]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TwistWithCovariance {
    pub twist: crate::geometry_msgs::msg::Twist,
    #[serde_as(as = "[_; 36]")]
    pub covariance: [f64; 36],
}

impl Default for TwistWithCovariance {
    fn default() -> Self {
        TwistWithCovariance {
            twist: crate::geometry_msgs::msg::Twist::default(),
            covariance: [0.0; 36],
        }
    }
}

impl ros2_client::Message for TwistWithCovariance {}
