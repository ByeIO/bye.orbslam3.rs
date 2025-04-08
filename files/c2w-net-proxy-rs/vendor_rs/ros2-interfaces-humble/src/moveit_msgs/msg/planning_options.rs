use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlanningOptions {
    pub planning_scene_diff: crate::moveit_msgs::msg::PlanningScene,
    pub plan_only: bool,
    pub look_around: bool,
    pub look_around_attempts: i32,
    pub max_safe_execution_cost: f64,
    pub replan: bool,
    pub replan_attempts: i32,
    pub replan_delay: f64,
}

impl Default for PlanningOptions {
    fn default() -> Self {
        PlanningOptions {
            planning_scene_diff: crate::moveit_msgs::msg::PlanningScene::default(),
            plan_only: false,
            look_around: false,
            look_around_attempts: 0,
            max_safe_execution_cost: 0.0,
            replan: false,
            replan_attempts: 0,
            replan_delay: 0.0,
        }
    }
}

impl ros2_client::Message for PlanningOptions {}
