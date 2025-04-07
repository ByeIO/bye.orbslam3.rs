use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GoalpostArray {
    pub header: crate::std_msgs::msg::Header,
    pub posts: Vec<crate::soccer_vision_3d_msgs::msg::Goalpost>,
}

impl Default for GoalpostArray {
    fn default() -> Self {
        GoalpostArray {
            header: crate::std_msgs::msg::Header::default(),
            posts: Vec::new(),
        }
    }
}

impl ros2_client::Message for GoalpostArray {}
