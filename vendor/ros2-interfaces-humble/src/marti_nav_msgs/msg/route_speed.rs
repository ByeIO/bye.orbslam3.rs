use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RouteSpeed {
    pub header: crate::std_msgs::msg::Header,
    pub id: ::std::string::String,
    pub distance: f32,
    pub speed: f32,
}

impl Default for RouteSpeed {
    fn default() -> Self {
        RouteSpeed {
            header: crate::std_msgs::msg::Header::default(),
            id: ::std::string::String::new(),
            distance: 0.0,
            speed: 0.0,
        }
    }
}

impl ros2_client::Message for RouteSpeed {}
