use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrajectoryWaypoint {
    pub time: i64,
    pub position: [f64; 3],
    pub velocity: [f64; 3],
}

impl Default for TrajectoryWaypoint {
    fn default() -> Self {
        TrajectoryWaypoint {
            time: 0,
            position: [0.0; 3],
            velocity: [0.0; 3],
        }
    }
}

impl ros2_client::Message for TrajectoryWaypoint {}
