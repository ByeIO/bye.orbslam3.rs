use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderArray {
    pub header: crate::std_msgs::msg::Header,
    pub orders: Vec<crate::tuw_multi_robot_msgs::msg::Order>,
}

impl Default for OrderArray {
    fn default() -> Self {
        OrderArray {
            header: crate::std_msgs::msg::Header::default(),
            orders: Vec::new(),
        }
    }
}

impl ros2_client::Message for OrderArray {}
