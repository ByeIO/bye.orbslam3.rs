use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Grasp {
    pub id: ::std::string::String,
    pub pre_grasp_posture: crate::trajectory_msgs::msg::JointTrajectory,
    pub grasp_posture: crate::trajectory_msgs::msg::JointTrajectory,
    pub grasp_pose: crate::geometry_msgs::msg::PoseStamped,
    pub grasp_quality: f64,
    pub pre_grasp_approach: crate::moveit_msgs::msg::GripperTranslation,
    pub post_grasp_retreat: crate::moveit_msgs::msg::GripperTranslation,
    pub post_place_retreat: crate::moveit_msgs::msg::GripperTranslation,
    pub max_contact_force: f32,
    pub allowed_touch_objects: Vec<::std::string::String>,
}

impl Default for Grasp {
    fn default() -> Self {
        Grasp {
            id: ::std::string::String::new(),
            pre_grasp_posture: crate::trajectory_msgs::msg::JointTrajectory::default(),
            grasp_posture: crate::trajectory_msgs::msg::JointTrajectory::default(),
            grasp_pose: crate::geometry_msgs::msg::PoseStamped::default(),
            grasp_quality: 0.0,
            pre_grasp_approach: crate::moveit_msgs::msg::GripperTranslation::default(),
            post_grasp_retreat: crate::moveit_msgs::msg::GripperTranslation::default(),
            post_place_retreat: crate::moveit_msgs::msg::GripperTranslation::default(),
            max_contact_force: 0.0,
            allowed_touch_objects: Vec::new(),
        }
    }
}

impl ros2_client::Message for Grasp {}
