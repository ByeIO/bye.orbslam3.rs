use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObjectB {
    pub can_id: u32,
    pub stamp: crate::builtin_interfaces::msg::Time,
    pub id: u8,
    pub time_since_meas: f64,
    pub zone: u8,
    pub rcs: f64,
    pub moving: bool,
    pub near: bool,
    pub exist_probability: f64,
}

impl Default for ObjectB {
    fn default() -> Self {
        ObjectB {
            can_id: 0,
            stamp: crate::builtin_interfaces::msg::Time::default(),
            id: 0,
            time_since_meas: 0.0,
            zone: 0,
            rcs: 0.0,
            moving: false,
            near: false,
            exist_probability: 0.0,
        }
    }
}

impl ros2_client::Message for ObjectB {}
