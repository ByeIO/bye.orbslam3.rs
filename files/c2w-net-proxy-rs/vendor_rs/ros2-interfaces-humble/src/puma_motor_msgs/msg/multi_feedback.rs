use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MultiFeedback {
    pub header: crate::std_msgs::msg::Header,
    pub drivers_feedback: Vec<crate::puma_motor_msgs::msg::Feedback>,
}

impl Default for MultiFeedback {
    fn default() -> Self {
        MultiFeedback {
            header: crate::std_msgs::msg::Header::default(),
            drivers_feedback: Vec::new(),
        }
    }
}

impl ros2_client::Message for MultiFeedback {}
