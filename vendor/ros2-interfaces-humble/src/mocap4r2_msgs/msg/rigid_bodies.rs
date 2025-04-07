use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RigidBodies {
    pub header: crate::std_msgs::msg::Header,
    pub frame_number: u32,
    pub rigidbodies: Vec<crate::mocap4r2_msgs::msg::RigidBody>,
}

impl Default for RigidBodies {
    fn default() -> Self {
        RigidBodies {
            header: crate::std_msgs::msg::Header::default(),
            frame_number: 0,
            rigidbodies: Vec::new(),
        }
    }
}

impl ros2_client::Message for RigidBodies {}
