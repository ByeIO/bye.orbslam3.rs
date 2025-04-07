use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pickup {
    pub header: crate::std_msgs::msg::Header,
    pub robot_name: ::std::string::String,
    pub order_id: i32,
}

impl Default for Pickup {
    fn default() -> Self {
        Pickup {
            header: crate::std_msgs::msg::Header::default(),
            robot_name: ::std::string::String::new(),
            order_id: 0,
        }
    }
}

impl ros2_client::Message for Pickup {}
