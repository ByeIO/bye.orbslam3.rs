use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Battery {
    pub header: crate::std_msgs::msg::Header,
    pub state_of_charge: f32,
    pub voltage: f32,
    pub current: f32,
    pub temperature: f32,
    pub ignition: crate::ds_dbw_msgs::msg::Ignition,
}

impl Default for Battery {
    fn default() -> Self {
        Battery {
            header: crate::std_msgs::msg::Header::default(),
            state_of_charge: 0.0,
            voltage: 0.0,
            current: 0.0,
            temperature: 0.0,
            ignition: crate::ds_dbw_msgs::msg::Ignition::default(),
        }
    }
}

impl ros2_client::Message for Battery {}
