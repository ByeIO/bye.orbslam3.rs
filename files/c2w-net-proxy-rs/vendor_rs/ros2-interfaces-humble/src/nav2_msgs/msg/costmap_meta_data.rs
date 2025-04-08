use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CostmapMetaData {
    pub map_load_time: crate::builtin_interfaces::msg::Time,
    pub update_time: crate::builtin_interfaces::msg::Time,
    pub layer: ::std::string::String,
    pub resolution: f32,
    pub size_x: u32,
    pub size_y: u32,
    pub origin: crate::geometry_msgs::msg::Pose,
}

impl Default for CostmapMetaData {
    fn default() -> Self {
        CostmapMetaData {
            map_load_time: crate::builtin_interfaces::msg::Time::default(),
            update_time: crate::builtin_interfaces::msg::Time::default(),
            layer: ::std::string::String::new(),
            resolution: 0.0,
            size_x: 0,
            size_y: 0,
            origin: crate::geometry_msgs::msg::Pose::default(),
        }
    }
}

impl ros2_client::Message for CostmapMetaData {}
