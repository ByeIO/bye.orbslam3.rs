use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RouteSegments {
    pub header: crate::std_msgs::msg::Header,
    pub segments: Vec<crate::tuw_nav_msgs::msg::RouteSegment>,
}

impl Default for RouteSegments {
    fn default() -> Self {
        RouteSegments {
            header: crate::std_msgs::msg::Header::default(),
            segments: Vec::new(),
        }
    }
}

impl ros2_client::Message for RouteSegments {}
