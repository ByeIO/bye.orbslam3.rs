use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Obstacle {
    pub header: crate::std_msgs::msg::Header,
    pub id: i32,
    pub source: ::std::string::String,
    pub level_name: ::std::string::String,
    pub classification: ::std::string::String,
    pub bbox: crate::rmf_obstacle_msgs::msg::BoundingBox3D,
    pub data_resolution: f64,
    pub data: Vec<i8>,
    pub lifetime: crate::builtin_interfaces::msg::Duration,
    pub action: i32,
}

impl Obstacle {
    pub const ACTION_ADD: i32 = 1;
    pub const ACTION_DELETE: i32 = 2;
    pub const ACTION_DELETEALL: i32 = 3;
}

impl Default for Obstacle {
    fn default() -> Self {
        Obstacle {
            header: crate::std_msgs::msg::Header::default(),
            id: 0,
            source: ::std::string::String::new(),
            level_name: ::std::string::String::new(),
            classification: ::std::string::String::new(),
            bbox: crate::rmf_obstacle_msgs::msg::BoundingBox3D::default(),
            data_resolution: 0.0,
            data: Vec::new(),
            lifetime: crate::builtin_interfaces::msg::Duration::default(),
            action: 0,
        }
    }
}

impl ros2_client::Message for Obstacle {}
