use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WaypointList {
    pub current_seq: u16,
    pub waypoints: Vec<crate::mavros_msgs::msg::Waypoint>,
}

impl Default for WaypointList {
    fn default() -> Self {
        WaypointList {
            current_seq: 0,
            waypoints: Vec::new(),
        }
    }
}

impl ros2_client::Message for WaypointList {}
