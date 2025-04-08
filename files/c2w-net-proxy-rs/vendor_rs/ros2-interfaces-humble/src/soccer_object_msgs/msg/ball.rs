use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ball {
    pub header: crate::std_msgs::msg::Header,
    pub center: crate::geometry_msgs::msg::Point,
}

impl Default for Ball {
    fn default() -> Self {
        Ball {
            header: crate::std_msgs::msg::Header::default(),
            center: crate::geometry_msgs::msg::Point::default(),
        }
    }
}

impl ros2_client::Message for Ball {}
