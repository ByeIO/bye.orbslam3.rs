use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarkerStamped {
    pub header: crate::std_msgs::msg::Header,
    pub marker: crate::marker_msgs::msg::Marker,
}

impl Default for MarkerStamped {
    fn default() -> Self {
        MarkerStamped {
            header: crate::std_msgs::msg::Header::default(),
            marker: crate::marker_msgs::msg::Marker::default(),
        }
    }
}

impl ros2_client::Message for MarkerStamped {}
