use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComplexPolygon2D {
    pub outer: crate::polygon_msgs::msg::Polygon2D,
    pub inner: Vec<crate::polygon_msgs::msg::Polygon2D>,
}

impl Default for ComplexPolygon2D {
    fn default() -> Self {
        ComplexPolygon2D {
            outer: crate::polygon_msgs::msg::Polygon2D::default(),
            inner: Vec::new(),
        }
    }
}

impl ros2_client::Message for ComplexPolygon2D {}
