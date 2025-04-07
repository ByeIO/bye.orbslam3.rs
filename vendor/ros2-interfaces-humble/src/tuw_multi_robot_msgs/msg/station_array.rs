use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StationArray {
    pub header: crate::std_msgs::msg::Header,
    pub stations: Vec<crate::tuw_multi_robot_msgs::msg::Station>,
}

impl Default for StationArray {
    fn default() -> Self {
        StationArray {
            header: crate::std_msgs::msg::Header::default(),
            stations: Vec::new(),
        }
    }
}

impl ros2_client::Message for StationArray {}
