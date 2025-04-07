use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RouteOffset {
    pub header: crate::std_msgs::msg::Header,
    pub relative_pose: crate::geometry_msgs::msg::Pose,
    pub route_position: crate::marti_nav_msgs::msg::RoutePosition,
}

impl Default for RouteOffset {
    fn default() -> Self {
        RouteOffset {
            header: crate::std_msgs::msg::Header::default(),
            relative_pose: crate::geometry_msgs::msg::Pose::default(),
            route_position: crate::marti_nav_msgs::msg::RoutePosition::default(),
        }
    }
}

impl ros2_client::Message for RouteOffset {}
