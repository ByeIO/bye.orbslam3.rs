use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SignalArray {
    pub header: crate::std_msgs::msg::Header,
    pub signals: Vec<crate::lgsvl_msgs::msg::Signal>,
}

impl Default for SignalArray {
    fn default() -> Self {
        SignalArray {
            header: crate::std_msgs::msg::Header::default(),
            signals: Vec::new(),
        }
    }
}

impl ros2_client::Message for SignalArray {}
