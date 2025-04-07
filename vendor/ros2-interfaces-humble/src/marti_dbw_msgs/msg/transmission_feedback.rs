use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransmissionFeedback {
    pub header: crate::std_msgs::msg::Header,
    pub current_range: ::std::string::String,
    pub stable: bool,
    pub reverse: bool,
    pub forward: bool,
}

impl Default for TransmissionFeedback {
    fn default() -> Self {
        TransmissionFeedback {
            header: crate::std_msgs::msg::Header::default(),
            current_range: ::std::string::String::new(),
            stable: false,
            reverse: false,
            forward: false,
        }
    }
}

impl ros2_client::Message for TransmissionFeedback {}
