use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RouteSegment {
    pub id: crate::unique_identifier_msgs::msg::UUID,
    pub start: crate::unique_identifier_msgs::msg::UUID,
    pub end: crate::unique_identifier_msgs::msg::UUID,
    pub props: Vec<crate::geographic_msgs::msg::KeyValue>,
}

impl Default for RouteSegment {
    fn default() -> Self {
        RouteSegment {
            id: crate::unique_identifier_msgs::msg::UUID::default(),
            start: crate::unique_identifier_msgs::msg::UUID::default(),
            end: crate::unique_identifier_msgs::msg::UUID::default(),
            props: Vec::new(),
        }
    }
}

impl ros2_client::Message for RouteSegment {}
