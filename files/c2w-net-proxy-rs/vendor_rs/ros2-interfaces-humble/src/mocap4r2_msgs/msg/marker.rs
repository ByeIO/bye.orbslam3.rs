use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Marker {
    pub id_type: i8,
    pub marker_index: i32,
    pub marker_name: ::std::string::String,
    pub translation: crate::geometry_msgs::msg::Point,
}

impl Marker {
    pub const USE_NAME: i8 = 0;
    pub const USE_INDEX: i8 = 1;
    pub const USE_BOTH: i8 = 2;
}

impl Default for Marker {
    fn default() -> Self {
        Marker {
            id_type: 0,
            marker_index: 0,
            marker_name: ::std::string::String::new(),
            translation: crate::geometry_msgs::msg::Point::default(),
        }
    }
}

impl ros2_client::Message for Marker {}
