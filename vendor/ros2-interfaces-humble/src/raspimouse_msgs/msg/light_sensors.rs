use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LightSensors {
    pub forward_r: i16,
    pub forward_l: i16,
    pub left: i16,
    pub right: i16,
}

impl Default for LightSensors {
    fn default() -> Self {
        LightSensors {
            forward_r: 0,
            forward_l: 0,
            left: 0,
            right: 0,
        }
    }
}

impl ros2_client::Message for LightSensors {}
