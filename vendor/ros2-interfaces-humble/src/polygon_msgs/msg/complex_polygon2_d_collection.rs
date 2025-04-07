use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComplexPolygon2DCollection {
    pub header: crate::std_msgs::msg::Header,
    pub polygons: Vec<crate::polygon_msgs::msg::ComplexPolygon2D>,
    pub colors: Vec<crate::std_msgs::msg::ColorRGBA>,
}

impl Default for ComplexPolygon2DCollection {
    fn default() -> Self {
        ComplexPolygon2DCollection {
            header: crate::std_msgs::msg::Header::default(),
            polygons: Vec::new(),
            colors: Vec::new(),
        }
    }
}

impl ros2_client::Message for ComplexPolygon2DCollection {}
