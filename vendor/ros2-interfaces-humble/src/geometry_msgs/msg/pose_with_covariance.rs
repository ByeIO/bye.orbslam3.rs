use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[serde_as]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PoseWithCovariance {
    pub pose: crate::geometry_msgs::msg::Pose,
    #[serde_as(as = "[_; 36]")]
    pub covariance: [f64; 36],
}

impl Default for PoseWithCovariance {
    fn default() -> Self {
        PoseWithCovariance {
            pose: crate::geometry_msgs::msg::Pose::default(),
            covariance: [0.0; 36],
        }
    }
}

impl ros2_client::Message for PoseWithCovariance {}
