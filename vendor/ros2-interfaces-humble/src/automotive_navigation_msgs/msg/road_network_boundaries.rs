use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RoadNetworkBoundaries {
    pub header: crate::std_msgs::msg::Header,
    pub road_network_boundaries: Vec<crate::automotive_navigation_msgs::msg::LaneBoundaryArray>,
}

impl Default for RoadNetworkBoundaries {
    fn default() -> Self {
        RoadNetworkBoundaries {
            header: crate::std_msgs::msg::Header::default(),
            road_network_boundaries: Vec::new(),
        }
    }
}

impl ros2_client::Message for RoadNetworkBoundaries {}
