use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NamedCollisions {
    pub header: crate::std_msgs::msg::Header,
    pub collisions: Vec<crate::collision_log_msgs::msg::NamedCollision>,
}

impl Default for NamedCollisions {
    fn default() -> Self {
        NamedCollisions {
            header: crate::std_msgs::msg::Header::default(),
            collisions: Vec::new(),
        }
    }
}

impl ros2_client::Message for NamedCollisions {}
