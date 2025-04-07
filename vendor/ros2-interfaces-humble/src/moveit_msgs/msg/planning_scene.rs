use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlanningScene {
    pub name: ::std::string::String,
    pub robot_state: crate::moveit_msgs::msg::RobotState,
    pub robot_model_name: ::std::string::String,
    pub fixed_frame_transforms: Vec<crate::geometry_msgs::msg::TransformStamped>,
    pub allowed_collision_matrix: crate::moveit_msgs::msg::AllowedCollisionMatrix,
    pub link_padding: Vec<crate::moveit_msgs::msg::LinkPadding>,
    pub link_scale: Vec<crate::moveit_msgs::msg::LinkScale>,
    pub object_colors: Vec<crate::moveit_msgs::msg::ObjectColor>,
    pub world: crate::moveit_msgs::msg::PlanningSceneWorld,
    pub is_diff: bool,
}

impl Default for PlanningScene {
    fn default() -> Self {
        PlanningScene {
            name: ::std::string::String::new(),
            robot_state: crate::moveit_msgs::msg::RobotState::default(),
            robot_model_name: ::std::string::String::new(),
            fixed_frame_transforms: Vec::new(),
            allowed_collision_matrix: crate::moveit_msgs::msg::AllowedCollisionMatrix::default(),
            link_padding: Vec::new(),
            link_scale: Vec::new(),
            object_colors: Vec::new(),
            world: crate::moveit_msgs::msg::PlanningSceneWorld::default(),
            is_diff: false,
        }
    }
}

impl ros2_client::Message for PlanningScene {}
