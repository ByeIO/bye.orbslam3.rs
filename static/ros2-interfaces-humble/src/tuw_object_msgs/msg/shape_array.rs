use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShapeArray {
    pub header: crate::std_msgs::msg::Header,
    pub shapes: Vec<crate::tuw_object_msgs::msg::Shape>,
}

impl Default for ShapeArray {
    fn default() -> Self {
        ShapeArray {
            header: crate::std_msgs::msg::Header::default(),
            shapes: Vec::new(),
        }
    }
}

impl ros2_client::Message for ShapeArray {}
