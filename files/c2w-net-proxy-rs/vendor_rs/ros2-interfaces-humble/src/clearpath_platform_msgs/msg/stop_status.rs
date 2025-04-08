use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StopStatus {
    pub header: crate::std_msgs::msg::Header,
    pub stop_power_status: bool,
    pub external_stop_present: bool,
    pub needs_reset: bool,
}

impl Default for StopStatus {
    fn default() -> Self {
        StopStatus {
            header: crate::std_msgs::msg::Header::default(),
            stop_power_status: false,
            external_stop_present: false,
            needs_reset: false,
        }
    }
}

impl ros2_client::Message for StopStatus {}
