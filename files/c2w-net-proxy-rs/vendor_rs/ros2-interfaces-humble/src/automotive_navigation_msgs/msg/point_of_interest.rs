use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PointOfInterest {
    pub guid: u64,
    pub latitude: f64,
    pub longitude: f64,
    pub params: ::std::string::String,
}

impl Default for PointOfInterest {
    fn default() -> Self {
        PointOfInterest {
            guid: 0,
            latitude: 0.0,
            longitude: 0.0,
            params: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for PointOfInterest {}
