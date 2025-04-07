use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CollisionDetection {
    pub gripper_id: ::std::string::String,
    pub pre_grasp_offset: crate::geometry_msgs::msg::Point,
}

impl Default for CollisionDetection {
    fn default() -> Self {
        CollisionDetection {
            gripper_id: ::std::string::String::new(),
            pre_grasp_offset: crate::geometry_msgs::msg::Point::default(),
        }
    }
}

impl ros2_client::Message for CollisionDetection {}
