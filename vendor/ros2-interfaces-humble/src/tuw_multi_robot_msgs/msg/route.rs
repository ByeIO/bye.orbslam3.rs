use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Route {
    pub header: crate::std_msgs::msg::Header,
    pub segments: Vec<crate::tuw_multi_robot_msgs::msg::RouteSegment>,
}

impl Default for Route {
    fn default() -> Self {
        Route {
            header: crate::std_msgs::msg::Header::default(),
            segments: Vec::new(),
        }
    }
}

impl ros2_client::Message for Route {}
