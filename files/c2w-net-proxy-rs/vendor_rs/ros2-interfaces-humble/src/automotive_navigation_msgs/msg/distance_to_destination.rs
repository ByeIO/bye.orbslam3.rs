use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DistanceToDestination {
    pub header: crate::std_msgs::msg::Header,
    pub msg_counter: u8,
    pub distance: f32,
}

impl Default for DistanceToDestination {
    fn default() -> Self {
        DistanceToDestination {
            header: crate::std_msgs::msg::Header::default(),
            msg_counter: 0,
            distance: 0.0,
        }
    }
}

impl ros2_client::Message for DistanceToDestination {}
