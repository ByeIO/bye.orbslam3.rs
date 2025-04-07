use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegionOfInterest2D {
    pub id: ::std::string::String,
    pub offset_x: u32,
    pub offset_y: u32,
    pub width: u32,
    pub height: u32,
}

impl Default for RegionOfInterest2D {
    fn default() -> Self {
        RegionOfInterest2D {
            id: ::std::string::String::new(),
            offset_x: 0,
            offset_y: 0,
            width: 0,
            height: 0,
        }
    }
}

impl ros2_client::Message for RegionOfInterest2D {}
