use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CaptureConfig {
    pub joint_states: crate::sensor_msgs::msg::JointState,
    pub features: Vec<::std::string::String>,
}

impl Default for CaptureConfig {
    fn default() -> Self {
        CaptureConfig {
            joint_states: crate::sensor_msgs::msg::JointState::default(),
            features: Vec::new(),
        }
    }
}

impl ros2_client::Message for CaptureConfig {}
