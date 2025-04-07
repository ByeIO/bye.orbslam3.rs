use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DesiredDestination {
    pub header: crate::std_msgs::msg::Header,
    pub msg_counter: u8,
    pub valid: u16,
    pub latitude: f64,
    pub longitude: f64,
}

impl Default for DesiredDestination {
    fn default() -> Self {
        DesiredDestination {
            header: crate::std_msgs::msg::Header::default(),
            msg_counter: 0,
            valid: 0,
            latitude: 0.0,
            longitude: 0.0,
        }
    }
}

impl ros2_client::Message for DesiredDestination {}
