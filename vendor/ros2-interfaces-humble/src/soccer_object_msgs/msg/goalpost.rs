use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Goalpost {
    pub header: crate::std_msgs::msg::Header,
    pub observed_top: bool,
    pub point: crate::geometry_msgs::msg::Point,
}

impl Default for Goalpost {
    fn default() -> Self {
        Goalpost {
            header: crate::std_msgs::msg::Header::default(),
            observed_top: false,
            point: crate::geometry_msgs::msg::Point::default(),
        }
    }
}

impl ros2_client::Message for Goalpost {}
