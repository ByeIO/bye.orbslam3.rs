use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectedMap {
    pub map: crate::nav_msgs::msg::OccupancyGrid,
    pub min_z: f64,
    pub max_z: f64,
}

impl Default for ProjectedMap {
    fn default() -> Self {
        ProjectedMap {
            map: crate::nav_msgs::msg::OccupancyGrid::default(),
            min_z: 0.0,
            max_z: 0.0,
        }
    }
}

impl ros2_client::Message for ProjectedMap {}
