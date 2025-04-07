use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Grasp {
    pub id: ::std::string::String,
    pub uuid: ::std::string::String,
    pub match_uuid: ::std::string::String,
    pub pose: crate::geometry_msgs::msg::PoseStamped,
    pub priority: i8,
    pub gripper_id: ::std::string::String,
    pub collision_checked: bool,
}

impl Default for Grasp {
    fn default() -> Self {
        Grasp {
            id: ::std::string::String::new(),
            uuid: ::std::string::String::new(),
            match_uuid: ::std::string::String::new(),
            pose: crate::geometry_msgs::msg::PoseStamped::default(),
            priority: 0,
            gripper_id: ::std::string::String::new(),
            collision_checked: false,
        }
    }
}

impl ros2_client::Message for Grasp {}
