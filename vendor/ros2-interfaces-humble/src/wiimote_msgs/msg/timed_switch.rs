use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimedSwitch {
    pub switch_mode: i8,
    pub num_cycles: i32,
    pub pulse_pattern: Vec<f32>,
}

impl TimedSwitch {
    pub const ON: i8 = 1;
    pub const OFF: i8 = 0;
    pub const NO_CHANGE: i8 = -2;
    pub const REPEAT: i8 = -1;
    pub const FOREVER: i8 = -1;
}

impl Default for TimedSwitch {
    fn default() -> Self {
        TimedSwitch {
            switch_mode: 0,
            num_cycles: 0,
            pulse_pattern: Vec::new(),
        }
    }
}

impl ros2_client::Message for TimedSwitch {}
