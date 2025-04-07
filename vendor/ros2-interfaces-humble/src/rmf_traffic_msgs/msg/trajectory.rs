use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Trajectory {
    pub waypoints: Vec<crate::rmf_traffic_msgs::msg::TrajectoryWaypoint>,
}

impl Default for Trajectory {
    fn default() -> Self {
        Trajectory {
            waypoints: Vec::new(),
        }
    }
}

impl ros2_client::Message for Trajectory {}
