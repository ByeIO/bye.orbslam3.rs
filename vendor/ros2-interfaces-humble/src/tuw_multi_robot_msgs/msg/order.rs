use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Order {
    pub header: crate::std_msgs::msg::Header,
    pub order_id: i32,
    pub order_name: ::std::string::String,
    pub stations: Vec<crate::tuw_multi_robot_msgs::msg::Station>,
}

impl Default for Order {
    fn default() -> Self {
        Order {
            header: crate::std_msgs::msg::Header::default(),
            order_id: 0,
            order_name: ::std::string::String::new(),
            stations: Vec::new(),
        }
    }
}

impl ros2_client::Message for Order {}
