use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Leds {
    pub led0: bool,
    pub led1: bool,
    pub led2: bool,
    pub led3: bool,
}

impl Default for Leds {
    fn default() -> Self {
        Leds {
            led0: false,
            led1: false,
            led2: false,
            led3: false,
        }
    }
}

impl ros2_client::Message for Leds {}
