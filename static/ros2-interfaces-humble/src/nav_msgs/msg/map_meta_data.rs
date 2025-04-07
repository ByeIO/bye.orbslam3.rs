use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MapMetaData {
    pub map_load_time: crate::builtin_interfaces::msg::Time,
    pub resolution: f32,
    pub width: u32,
    pub height: u32,
    pub origin: crate::geometry_msgs::msg::Pose,
}

impl Default for MapMetaData {
    fn default() -> Self {
        MapMetaData {
            map_load_time: crate::builtin_interfaces::msg::Time::default(),
            resolution: 0.0,
            width: 0,
            height: 0,
            origin: crate::geometry_msgs::msg::Pose::default(),
        }
    }
}

impl ros2_client::Message for MapMetaData {}
