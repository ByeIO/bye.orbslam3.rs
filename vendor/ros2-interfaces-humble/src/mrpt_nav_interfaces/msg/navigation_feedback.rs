use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NavigationFeedback {
    pub total_waypoints: i16,
    pub reached_waypoints: i16,
}

impl Default for NavigationFeedback {
    fn default() -> Self {
        NavigationFeedback {
            total_waypoints: 0,
            reached_waypoints: 0,
        }
    }
}

impl ros2_client::Message for NavigationFeedback {}
