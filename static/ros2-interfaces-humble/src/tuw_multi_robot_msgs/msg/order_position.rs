use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderPosition {
    pub header: crate::std_msgs::msg::Header,
    pub order_id: i32,
    pub position: crate::geometry_msgs::msg::Pose,
}

impl Default for OrderPosition {
    fn default() -> Self {
        OrderPosition {
            header: crate::std_msgs::msg::Header::default(),
            order_id: 0,
            position: crate::geometry_msgs::msg::Pose::default(),
        }
    }
}

impl ros2_client::Message for OrderPosition {}
