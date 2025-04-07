use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeoPose {
    pub position: crate::geographic_msgs::msg::GeoPoint,
    pub orientation: crate::geometry_msgs::msg::Quaternion,
}

impl Default for GeoPose {
    fn default() -> Self {
        GeoPose {
            position: crate::geographic_msgs::msg::GeoPoint::default(),
            orientation: crate::geometry_msgs::msg::Quaternion::default(),
        }
    }
}

impl ros2_client::Message for GeoPose {}
