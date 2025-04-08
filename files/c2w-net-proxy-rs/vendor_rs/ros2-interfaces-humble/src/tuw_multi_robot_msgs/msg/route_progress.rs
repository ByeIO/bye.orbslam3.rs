use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RouteProgress {
    pub header: crate::std_msgs::msg::Header,
    pub passed: Vec<i32>,
    pub current: i32,
    pub todo: Vec<i32>,
    pub progress: f32,
}

impl Default for RouteProgress {
    fn default() -> Self {
        RouteProgress {
            header: crate::std_msgs::msg::Header::default(),
            passed: Vec::new(),
            current: 0,
            todo: Vec::new(),
            progress: 0.0,
        }
    }
}

impl ros2_client::Message for RouteProgress {}
