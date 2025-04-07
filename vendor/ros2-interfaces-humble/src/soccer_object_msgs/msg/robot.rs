use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Robot {
    pub header: crate::std_msgs::msg::Header,
    pub head: crate::geometry_msgs::msg::Point,
    pub team: ::std::string::String,
    pub id: i32,
}

impl Default for Robot {
    fn default() -> Self {
        Robot {
            header: crate::std_msgs::msg::Header::default(),
            head: crate::geometry_msgs::msg::Point::default(),
            team: ::std::string::String::new(),
            id: 0,
        }
    }
}

impl ros2_client::Message for Robot {}
