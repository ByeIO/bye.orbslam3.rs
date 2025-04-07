use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Vector3Stamped {
    pub header: crate::std_msgs::msg::Header,
    pub vector: crate::geometry_msgs::msg::Vector3,
}

impl Default for Vector3Stamped {
    fn default() -> Self {
        Vector3Stamped {
            header: crate::std_msgs::msg::Header::default(),
            vector: crate::geometry_msgs::msg::Vector3::default(),
        }
    }
}

impl ros2_client::Message for Vector3Stamped {}
