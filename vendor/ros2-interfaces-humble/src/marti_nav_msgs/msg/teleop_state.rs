use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TeleopState {
    pub header: crate::std_msgs::msg::Header,
    pub teleop_signals: Vec<i32>,
}

impl Default for TeleopState {
    fn default() -> Self {
        TeleopState {
            header: crate::std_msgs::msg::Header::default(),
            teleop_signals: Vec::new(),
        }
    }
}

impl ros2_client::Message for TeleopState {}
