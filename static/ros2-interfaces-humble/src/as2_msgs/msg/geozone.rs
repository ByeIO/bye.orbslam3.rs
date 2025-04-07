use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Geozone {
    pub id: i8,
    pub alert: i8,
    #[serde(rename = "type")]    pub type_: ::std::string::String,
    pub data_type: ::std::string::String,
    pub polygon: crate::geometry_msgs::msg::Polygon,
    pub z_up: f32,
    pub z_down: f32,
}

impl Default for Geozone {
    fn default() -> Self {
        Geozone {
            id: 0,
            alert: 0,
            type_: ::std::string::String::new(),
            data_type: ::std::string::String::new(),
            polygon: crate::geometry_msgs::msg::Polygon::default(),
            z_up: 0.0,
            z_down: 0.0,
        }
    }
}

impl ros2_client::Message for Geozone {}
