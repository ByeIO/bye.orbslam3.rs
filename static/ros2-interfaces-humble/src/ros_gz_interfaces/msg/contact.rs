use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Contact {
    pub collision1: crate::ros_gz_interfaces::msg::Entity,
    pub collision2: crate::ros_gz_interfaces::msg::Entity,
    pub positions: Vec<crate::geometry_msgs::msg::Vector3>,
    pub normals: Vec<crate::geometry_msgs::msg::Vector3>,
    pub depths: Vec<f64>,
    pub wrenches: Vec<crate::ros_gz_interfaces::msg::JointWrench>,
}

impl Default for Contact {
    fn default() -> Self {
        Contact {
            collision1: crate::ros_gz_interfaces::msg::Entity::default(),
            collision2: crate::ros_gz_interfaces::msg::Entity::default(),
            positions: Vec::new(),
            normals: Vec::new(),
            depths: Vec::new(),
            wrenches: Vec::new(),
        }
    }
}

impl ros2_client::Message for Contact {}
