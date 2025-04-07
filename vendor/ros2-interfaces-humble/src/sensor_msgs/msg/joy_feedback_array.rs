use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JoyFeedbackArray {
    pub array: Vec<crate::sensor_msgs::msg::JoyFeedback>,
}

impl Default for JoyFeedbackArray {
    fn default() -> Self {
        JoyFeedbackArray {
            array: Vec::new(),
        }
    }
}

impl ros2_client::Message for JoyFeedbackArray {}
