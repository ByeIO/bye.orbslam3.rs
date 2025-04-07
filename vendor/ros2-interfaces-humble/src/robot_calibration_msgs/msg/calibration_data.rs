use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CalibrationData {
    pub joint_states: crate::sensor_msgs::msg::JointState,
    pub observations: Vec<crate::robot_calibration_msgs::msg::Observation>,
}

impl Default for CalibrationData {
    fn default() -> Self {
        CalibrationData {
            joint_states: crate::sensor_msgs::msg::JointState::default(),
            observations: Vec::new(),
        }
    }
}

impl ros2_client::Message for CalibrationData {}
