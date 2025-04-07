use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PoseWithUuidStamped {
    pub header: crate::std_msgs::msg::Header,
    pub pose: crate::geometry_msgs::msg::Pose,
    pub uuid: crate::unique_identifier_msgs::msg::UUID,
}

impl Default for PoseWithUuidStamped {
    fn default() -> Self {
        PoseWithUuidStamped {
            header: crate::std_msgs::msg::Header::default(),
            pose: crate::geometry_msgs::msg::Pose::default(),
            uuid: crate::unique_identifier_msgs::msg::UUID::default(),
        }
    }
}

impl ros2_client::Message for PoseWithUuidStamped {}
