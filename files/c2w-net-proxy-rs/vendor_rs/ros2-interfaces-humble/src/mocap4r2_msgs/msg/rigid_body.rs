use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RigidBody {
    pub rigid_body_name: ::std::string::String,
    pub markers: Vec<crate::mocap4r2_msgs::msg::Marker>,
    pub pose: crate::geometry_msgs::msg::Pose,
}

impl Default for RigidBody {
    fn default() -> Self {
        RigidBody {
            rigid_body_name: ::std::string::String::new(),
            markers: Vec::new(),
            pose: crate::geometry_msgs::msg::Pose::default(),
        }
    }
}

impl ros2_client::Message for RigidBody {}
