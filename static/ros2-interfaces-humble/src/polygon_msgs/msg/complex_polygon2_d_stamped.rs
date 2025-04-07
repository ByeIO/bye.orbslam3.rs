use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComplexPolygon2DStamped {
    pub header: crate::std_msgs::msg::Header,
    pub polygon: crate::polygon_msgs::msg::ComplexPolygon2D,
}

impl Default for ComplexPolygon2DStamped {
    fn default() -> Self {
        ComplexPolygon2DStamped {
            header: crate::std_msgs::msg::Header::default(),
            polygon: crate::polygon_msgs::msg::ComplexPolygon2D::default(),
        }
    }
}

impl ros2_client::Message for ComplexPolygon2DStamped {}
