use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RouteArray {
    pub header: crate::std_msgs::msg::Header,
    pub routes: Vec<crate::marti_nav_msgs::msg::Route>,
    pub properties: Vec<crate::marti_common_msgs::msg::KeyValue>,
}

impl Default for RouteArray {
    fn default() -> Self {
        RouteArray {
            header: crate::std_msgs::msg::Header::default(),
            routes: Vec::new(),
            properties: Vec::new(),
        }
    }
}

impl ros2_client::Message for RouteArray {}
