use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RoomsData {
    pub header: crate::std_msgs::msg::Header,
    pub rooms: Vec<crate::situational_graphs_msgs::msg::RoomData>,
}

impl Default for RoomsData {
    fn default() -> Self {
        RoomsData {
            header: crate::std_msgs::msg::Header::default(),
            rooms: Vec::new(),
        }
    }
}

impl ros2_client::Message for RoomsData {}
