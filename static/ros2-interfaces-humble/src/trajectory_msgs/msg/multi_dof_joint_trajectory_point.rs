use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MultiDOFJointTrajectoryPoint {
    pub transforms: Vec<crate::geometry_msgs::msg::Transform>,
    pub velocities: Vec<crate::geometry_msgs::msg::Twist>,
    pub accelerations: Vec<crate::geometry_msgs::msg::Twist>,
    pub time_from_start: crate::builtin_interfaces::msg::Duration,
}

impl Default for MultiDOFJointTrajectoryPoint {
    fn default() -> Self {
        MultiDOFJointTrajectoryPoint {
            transforms: Vec::new(),
            velocities: Vec::new(),
            accelerations: Vec::new(),
            time_from_start: crate::builtin_interfaces::msg::Duration::default(),
        }
    }
}

impl ros2_client::Message for MultiDOFJointTrajectoryPoint {}
