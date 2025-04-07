use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pedestrians {
    pub header: crate::std_msgs::msg::Header,
    pub pedestrians: Vec<crate::social_nav_msgs::msg::Pedestrian>,
}

impl Default for Pedestrians {
    fn default() -> Self {
        Pedestrians {
            header: crate::std_msgs::msg::Header::default(),
            pedestrians: Vec::new(),
        }
    }
}

impl ros2_client::Message for Pedestrians {}
