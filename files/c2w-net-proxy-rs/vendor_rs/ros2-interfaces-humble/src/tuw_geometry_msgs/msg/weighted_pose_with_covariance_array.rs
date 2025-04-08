use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WeightedPoseWithCovarianceArray {
    pub header: crate::std_msgs::msg::Header,
    pub poses: Vec<crate::tuw_geometry_msgs::msg::WeightedPoseWithCovariance>,
}

impl Default for WeightedPoseWithCovarianceArray {
    fn default() -> Self {
        WeightedPoseWithCovarianceArray {
            header: crate::std_msgs::msg::Header::default(),
            poses: Vec::new(),
        }
    }
}

impl ros2_client::Message for WeightedPoseWithCovarianceArray {}
