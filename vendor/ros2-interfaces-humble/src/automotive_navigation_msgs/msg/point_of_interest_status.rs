use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PointOfInterestStatus {
    pub guid: u64,
    pub distance: f32,
    pub heading: f32,
    pub x_position: f32,
    pub y_position: f32,
    pub params: ::std::string::String,
}

impl Default for PointOfInterestStatus {
    fn default() -> Self {
        PointOfInterestStatus {
            guid: 0,
            distance: 0.0,
            heading: 0.0,
            x_position: 0.0,
            y_position: 0.0,
            params: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for PointOfInterestStatus {}
