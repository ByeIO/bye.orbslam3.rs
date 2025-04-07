use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyPoint {
    pub pt: crate::rtabmap_msgs::msg::Point2f,
    pub size: f32,
    pub angle: f32,
    pub response: f32,
    pub octave: i32,
    pub class_id: i32,
}

impl Default for KeyPoint {
    fn default() -> Self {
        KeyPoint {
            pt: crate::rtabmap_msgs::msg::Point2f::default(),
            size: 0.0,
            angle: 0.0,
            response: 0.0,
            octave: 0,
            class_id: 0,
        }
    }
}

impl ros2_client::Message for KeyPoint {}
