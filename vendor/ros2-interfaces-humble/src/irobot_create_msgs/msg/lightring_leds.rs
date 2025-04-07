use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LightringLeds {
    pub header: crate::std_msgs::msg::Header,
    pub leds: [crate::irobot_create_msgs::msg::LedColor; 6],
    pub override_system: bool,
}

impl Default for LightringLeds {
    fn default() -> Self {
        LightringLeds {
            header: crate::std_msgs::msg::Header::default(),
            leds: core::array::from_fn(|_| crate::irobot_create_msgs::msg::LedColor::default()),
            override_system: false,
        }
    }
}

impl ros2_client::Message for LightringLeds {}
