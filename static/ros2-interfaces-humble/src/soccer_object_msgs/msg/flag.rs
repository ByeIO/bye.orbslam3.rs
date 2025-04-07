use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Flag {
    pub header: crate::std_msgs::msg::Header,
    pub base: crate::geometry_msgs::msg::Point,
}

impl Default for Flag {
    fn default() -> Self {
        Flag {
            header: crate::std_msgs::msg::Header::default(),
            base: crate::geometry_msgs::msg::Point::default(),
        }
    }
}

impl ros2_client::Message for Flag {}
