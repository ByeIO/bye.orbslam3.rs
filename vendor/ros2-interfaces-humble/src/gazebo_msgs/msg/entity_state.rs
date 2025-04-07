use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntityState {
    pub name: ::std::string::String,
    pub pose: crate::geometry_msgs::msg::Pose,
    pub twist: crate::geometry_msgs::msg::Twist,
    pub reference_frame: ::std::string::String,
}

impl Default for EntityState {
    fn default() -> Self {
        EntityState {
            name: ::std::string::String::new(),
            pose: crate::geometry_msgs::msg::Pose::default(),
            twist: crate::geometry_msgs::msg::Twist::default(),
            reference_frame: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for EntityState {}
