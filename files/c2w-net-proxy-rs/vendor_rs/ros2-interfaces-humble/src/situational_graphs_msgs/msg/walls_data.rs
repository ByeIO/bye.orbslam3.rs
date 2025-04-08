use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WallsData {
    pub header: crate::std_msgs::msg::Header,
    pub walls: Vec<crate::situational_graphs_msgs::msg::WallData>,
}

impl Default for WallsData {
    fn default() -> Self {
        WallsData {
            header: crate::std_msgs::msg::Header::default(),
            walls: Vec::new(),
        }
    }
}

impl ros2_client::Message for WallsData {}
