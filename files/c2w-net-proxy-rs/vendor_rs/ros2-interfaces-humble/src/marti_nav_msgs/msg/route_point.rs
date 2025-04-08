use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RoutePoint {
    pub pose: crate::geometry_msgs::msg::Pose,
    pub id: ::std::string::String,
    pub properties: Vec<crate::marti_common_msgs::msg::KeyValue>,
}

impl Default for RoutePoint {
    fn default() -> Self {
        RoutePoint {
            pose: crate::geometry_msgs::msg::Pose::default(),
            id: ::std::string::String::new(),
            properties: Vec::new(),
        }
    }
}

impl ros2_client::Message for RoutePoint {}
