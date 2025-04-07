use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JointTrajectoryPoint {
    pub positions: Vec<f64>,
    pub velocities: Vec<f64>,
    pub accelerations: Vec<f64>,
    pub effort: Vec<f64>,
    pub time_from_start: crate::builtin_interfaces::msg::Duration,
}

impl Default for JointTrajectoryPoint {
    fn default() -> Self {
        JointTrajectoryPoint {
            positions: Vec::new(),
            velocities: Vec::new(),
            accelerations: Vec::new(),
            effort: Vec::new(),
            time_from_start: crate::builtin_interfaces::msg::Duration::default(),
        }
    }
}

impl ros2_client::Message for JointTrajectoryPoint {}
