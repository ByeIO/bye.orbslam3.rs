use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlanningSceneWorld {
    pub collision_objects: Vec<crate::moveit_msgs::msg::CollisionObject>,
    pub octomap: crate::octomap_msgs::msg::OctomapWithPose,
}

impl Default for PlanningSceneWorld {
    fn default() -> Self {
        PlanningSceneWorld {
            collision_objects: Vec::new(),
            octomap: crate::octomap_msgs::msg::OctomapWithPose::default(),
        }
    }
}

impl ros2_client::Message for PlanningSceneWorld {}
