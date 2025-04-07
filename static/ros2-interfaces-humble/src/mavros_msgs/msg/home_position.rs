use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HomePosition {
    pub header: crate::std_msgs::msg::Header,
    pub geo: crate::geographic_msgs::msg::GeoPoint,
    pub position: crate::geometry_msgs::msg::Point,
    pub orientation: crate::geometry_msgs::msg::Quaternion,
    pub approach: crate::geometry_msgs::msg::Vector3,
}

impl Default for HomePosition {
    fn default() -> Self {
        HomePosition {
            header: crate::std_msgs::msg::Header::default(),
            geo: crate::geographic_msgs::msg::GeoPoint::default(),
            position: crate::geometry_msgs::msg::Point::default(),
            orientation: crate::geometry_msgs::msg::Quaternion::default(),
            approach: crate::geometry_msgs::msg::Vector3::default(),
        }
    }
}

impl ros2_client::Message for HomePosition {}
