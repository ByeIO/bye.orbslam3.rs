use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeographicMap {
    pub header: crate::std_msgs::msg::Header,
    pub id: crate::unique_identifier_msgs::msg::UUID,
    pub bounds: crate::geographic_msgs::msg::BoundingBox,
    pub points: Vec<crate::geographic_msgs::msg::WayPoint>,
    pub features: Vec<crate::geographic_msgs::msg::MapFeature>,
    pub props: Vec<crate::geographic_msgs::msg::KeyValue>,
}

impl Default for GeographicMap {
    fn default() -> Self {
        GeographicMap {
            header: crate::std_msgs::msg::Header::default(),
            id: crate::unique_identifier_msgs::msg::UUID::default(),
            bounds: crate::geographic_msgs::msg::BoundingBox::default(),
            points: Vec::new(),
            features: Vec::new(),
            props: Vec::new(),
        }
    }
}

impl ros2_client::Message for GeographicMap {}
