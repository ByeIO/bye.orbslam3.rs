use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImusInfo {
    pub sensor_ids: Vec<::std::string::String>,
    pub battery_level: f32,
    pub temperature: f32,
}

impl Default for ImusInfo {
    fn default() -> Self {
        ImusInfo {
            sensor_ids: Vec::new(),
            battery_level: 0.0,
            temperature: 0.0,
        }
    }
}

impl ros2_client::Message for ImusInfo {}
