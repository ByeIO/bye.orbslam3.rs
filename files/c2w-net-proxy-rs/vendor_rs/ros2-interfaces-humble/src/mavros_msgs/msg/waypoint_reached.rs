use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WaypointReached {
    pub header: crate::std_msgs::msg::Header,
    pub wp_seq: u16,
}

impl Default for WaypointReached {
    fn default() -> Self {
        WaypointReached {
            header: crate::std_msgs::msg::Header::default(),
            wp_seq: 0,
        }
    }
}

impl ros2_client::Message for WaypointReached {}
