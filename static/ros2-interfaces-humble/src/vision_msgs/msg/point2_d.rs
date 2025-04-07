use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

impl Default for Point2D {
    fn default() -> Self {
        Point2D {
            x: 0.0,
            y: 0.0,
        }
    }
}

impl ros2_client::Message for Point2D {}
