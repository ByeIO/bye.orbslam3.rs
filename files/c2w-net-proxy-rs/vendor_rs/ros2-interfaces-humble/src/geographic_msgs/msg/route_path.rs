use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RoutePath {
    pub header: crate::std_msgs::msg::Header,
    pub network: crate::unique_identifier_msgs::msg::UUID,
    pub segments: Vec<crate::unique_identifier_msgs::msg::UUID>,
    pub props: Vec<crate::geographic_msgs::msg::KeyValue>,
}

impl Default for RoutePath {
    fn default() -> Self {
        RoutePath {
            header: crate::std_msgs::msg::Header::default(),
            network: crate::unique_identifier_msgs::msg::UUID::default(),
            segments: Vec::new(),
            props: Vec::new(),
        }
    }
}

impl ros2_client::Message for RoutePath {}
