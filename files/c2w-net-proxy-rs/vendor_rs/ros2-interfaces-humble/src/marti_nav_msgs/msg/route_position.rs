use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RoutePosition {
    pub header: crate::std_msgs::msg::Header,
    pub route_id: ::std::string::String,
    pub id: ::std::string::String,
    pub distance: f32,
}

impl Default for RoutePosition {
    fn default() -> Self {
        RoutePosition {
            header: crate::std_msgs::msg::Header::default(),
            route_id: ::std::string::String::new(),
            id: ::std::string::String::new(),
            distance: 0.0,
        }
    }
}

impl ros2_client::Message for RoutePosition {}
