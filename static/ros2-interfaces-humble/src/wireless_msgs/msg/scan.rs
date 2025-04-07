use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Scan {
    pub networks: Vec<crate::wireless_msgs::msg::Network>,
}

impl Default for Scan {
    fn default() -> Self {
        Scan {
            networks: Vec::new(),
        }
    }
}

impl ros2_client::Message for Scan {}
