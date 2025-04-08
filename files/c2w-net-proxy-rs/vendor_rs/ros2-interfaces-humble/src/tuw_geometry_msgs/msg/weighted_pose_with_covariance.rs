use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[serde_as]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WeightedPoseWithCovariance {
    pub pose: crate::geometry_msgs::msg::Pose,
    #[serde_as(as = "[_; 36]")]
    pub covariance: [f64; 36],
    pub weight: f64,
}

impl Default for WeightedPoseWithCovariance {
    fn default() -> Self {
        WeightedPoseWithCovariance {
            pose: crate::geometry_msgs::msg::Pose::default(),
            covariance: [0.0; 36],
            weight: 0.0,
        }
    }
}

impl ros2_client::Message for WeightedPoseWithCovariance {}
