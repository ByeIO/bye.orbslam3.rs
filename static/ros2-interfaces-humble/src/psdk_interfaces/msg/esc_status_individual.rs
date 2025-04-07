use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EscStatusIndividual {
    pub current: i16,
    pub speed: i16,
    pub voltage: u16,
    pub temperature: i16,
    pub stall: i16,
    pub empty: i16,
    pub unbalanced: i16,
    pub esc_disconnected: i16,
    pub temperature_high: i16,
}

impl Default for EscStatusIndividual {
    fn default() -> Self {
        EscStatusIndividual {
            current: 0,
            speed: 0,
            voltage: 0,
            temperature: 0,
            stall: 0,
            empty: 0,
            unbalanced: 0,
            esc_disconnected: 0,
            temperature_high: 0,
        }
    }
}

impl ros2_client::Message for EscStatusIndividual {}
