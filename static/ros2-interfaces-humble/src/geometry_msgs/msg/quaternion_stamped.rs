use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuaternionStamped {
    pub header: crate::std_msgs::msg::Header,
    pub quaternion: crate::geometry_msgs::msg::Quaternion,
}

impl Default for QuaternionStamped {
    fn default() -> Self {
        QuaternionStamped {
            header: crate::std_msgs::msg::Header::default(),
            quaternion: crate::geometry_msgs::msg::Quaternion::default(),
        }
    }
}

impl ros2_client::Message for QuaternionStamped {}
