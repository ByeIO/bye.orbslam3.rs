use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlanesData {
    pub header: crate::std_msgs::msg::Header,
    pub x_planes: Vec<crate::situational_graphs_msgs::msg::PlaneData>,
    pub y_planes: Vec<crate::situational_graphs_msgs::msg::PlaneData>,
}

impl Default for PlanesData {
    fn default() -> Self {
        PlanesData {
            header: crate::std_msgs::msg::Header::default(),
            x_planes: Vec::new(),
            y_planes: Vec::new(),
        }
    }
}

impl ros2_client::Message for PlanesData {}
