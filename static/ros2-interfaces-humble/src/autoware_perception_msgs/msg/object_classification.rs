use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObjectClassification {
    pub label: u8,
    pub probability: f32,
}

impl ObjectClassification {
    pub const UNKNOWN: u8 = 0;
    pub const CAR: u8 = 1;
    pub const TRUCK: u8 = 2;
    pub const BUS: u8 = 3;
    pub const TRAILER: u8 = 4;
    pub const MOTORCYCLE: u8 = 5;
    pub const BICYCLE: u8 = 6;
    pub const PEDESTRIAN: u8 = 7;
}

impl Default for ObjectClassification {
    fn default() -> Self {
        ObjectClassification {
            label: 0,
            probability: 0.0,
        }
    }
}

impl ros2_client::Message for ObjectClassification {}
