use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeoPointStamped {
    pub header: crate::std_msgs::msg::Header,
    pub position: crate::geographic_msgs::msg::GeoPoint,
}

impl Default for GeoPointStamped {
    fn default() -> Self {
        GeoPointStamped {
            header: crate::std_msgs::msg::Header::default(),
            position: crate::geographic_msgs::msg::GeoPoint::default(),
        }
    }
}

impl ros2_client::Message for GeoPointStamped {}
