use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntityWrench {
    pub header: crate::std_msgs::msg::Header,
    pub entity: crate::ros_gz_interfaces::msg::Entity,
    pub wrench: crate::geometry_msgs::msg::Wrench,
}

impl Default for EntityWrench {
    fn default() -> Self {
        EntityWrench {
            header: crate::std_msgs::msg::Header::default(),
            entity: crate::ros_gz_interfaces::msg::Entity::default(),
            wrench: crate::geometry_msgs::msg::Wrench::default(),
        }
    }
}

impl ros2_client::Message for EntityWrench {}
