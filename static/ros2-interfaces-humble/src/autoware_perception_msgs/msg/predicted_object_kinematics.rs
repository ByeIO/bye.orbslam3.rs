use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PredictedObjectKinematics {
    pub initial_pose_with_covariance: crate::geometry_msgs::msg::PoseWithCovariance,
    pub initial_twist_with_covariance: crate::geometry_msgs::msg::TwistWithCovariance,
    pub initial_acceleration_with_covariance: crate::geometry_msgs::msg::AccelWithCovariance,
    pub predicted_paths: Vec<crate::autoware_perception_msgs::msg::PredictedPath>,
}

impl Default for PredictedObjectKinematics {
    fn default() -> Self {
        PredictedObjectKinematics {
            initial_pose_with_covariance: crate::geometry_msgs::msg::PoseWithCovariance::default(),
            initial_twist_with_covariance: crate::geometry_msgs::msg::TwistWithCovariance::default(),
            initial_acceleration_with_covariance: crate::geometry_msgs::msg::AccelWithCovariance::default(),
            predicted_paths: Vec::new(),
        }
    }
}

impl ros2_client::Message for PredictedObjectKinematics {}
