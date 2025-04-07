use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GenericTrajectory {
    pub header: crate::std_msgs::msg::Header,
    pub joint_trajectory: Vec<crate::trajectory_msgs::msg::JointTrajectory>,
    pub cartesian_trajectory: Vec<crate::moveit_msgs::msg::CartesianTrajectory>,
}

impl Default for GenericTrajectory {
    fn default() -> Self {
        GenericTrajectory {
            header: crate::std_msgs::msg::Header::default(),
            joint_trajectory: Vec::new(),
            cartesian_trajectory: Vec::new(),
        }
    }
}

impl ros2_client::Message for GenericTrajectory {}
