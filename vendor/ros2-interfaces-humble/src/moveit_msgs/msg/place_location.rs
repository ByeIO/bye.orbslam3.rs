use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlaceLocation {
    pub id: ::std::string::String,
    pub post_place_posture: crate::trajectory_msgs::msg::JointTrajectory,
    pub place_pose: crate::geometry_msgs::msg::PoseStamped,
    pub quality: f64,
    pub pre_place_approach: crate::moveit_msgs::msg::GripperTranslation,
    pub post_place_retreat: crate::moveit_msgs::msg::GripperTranslation,
    pub allowed_touch_objects: Vec<::std::string::String>,
}

impl Default for PlaceLocation {
    fn default() -> Self {
        PlaceLocation {
            id: ::std::string::String::new(),
            post_place_posture: crate::trajectory_msgs::msg::JointTrajectory::default(),
            place_pose: crate::geometry_msgs::msg::PoseStamped::default(),
            quality: 0.0,
            pre_place_approach: crate::moveit_msgs::msg::GripperTranslation::default(),
            post_place_retreat: crate::moveit_msgs::msg::GripperTranslation::default(),
            allowed_touch_objects: Vec::new(),
        }
    }
}

impl ros2_client::Message for PlaceLocation {}
