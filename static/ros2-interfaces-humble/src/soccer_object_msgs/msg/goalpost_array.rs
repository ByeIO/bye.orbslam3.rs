use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GoalpostArray {
    pub posts: Vec<crate::soccer_object_msgs::msg::Goalpost>,
}

impl Default for GoalpostArray {
    fn default() -> Self {
        GoalpostArray {
            posts: Vec::new(),
        }
    }
}

impl ros2_client::Message for GoalpostArray {}
