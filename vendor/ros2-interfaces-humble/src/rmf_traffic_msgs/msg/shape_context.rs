use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShapeContext {
    pub convex_shapes: crate::rmf_traffic_msgs::msg::ConvexShapeContext,
}

impl Default for ShapeContext {
    fn default() -> Self {
        ShapeContext {
            convex_shapes: crate::rmf_traffic_msgs::msg::ConvexShapeContext::default(),
        }
    }
}

impl ros2_client::Message for ShapeContext {}
