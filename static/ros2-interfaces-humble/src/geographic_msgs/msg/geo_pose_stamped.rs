use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeoPoseStamped {
    pub header: crate::std_msgs::msg::Header,
    pub pose: crate::geographic_msgs::msg::GeoPose,
}

impl Default for GeoPoseStamped {
    fn default() -> Self {
        GeoPoseStamped {
            header: crate::std_msgs::msg::Header::default(),
            pose: crate::geographic_msgs::msg::GeoPose::default(),
        }
    }
}

impl ros2_client::Message for GeoPoseStamped {}
