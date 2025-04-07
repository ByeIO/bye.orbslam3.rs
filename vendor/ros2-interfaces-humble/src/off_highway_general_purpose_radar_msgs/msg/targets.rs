use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Targets {
    pub header: crate::std_msgs::msg::Header,
    pub targets: Vec<crate::off_highway_general_purpose_radar_msgs::msg::Target>,
}

impl Default for Targets {
    fn default() -> Self {
        Targets {
            header: crate::std_msgs::msg::Header::default(),
            targets: Vec::new(),
        }
    }
}

impl ros2_client::Message for Targets {}
