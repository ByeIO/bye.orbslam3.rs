use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NegotiationStatus {
    pub conflict_version: u64,
    pub participants: Vec<u64>,
    pub start_time: crate::builtin_interfaces::msg::Time,
    pub last_response_time: crate::builtin_interfaces::msg::Time,
}

impl Default for NegotiationStatus {
    fn default() -> Self {
        NegotiationStatus {
            conflict_version: 0,
            participants: Vec::new(),
            start_time: crate::builtin_interfaces::msg::Time::default(),
            last_response_time: crate::builtin_interfaces::msg::Time::default(),
        }
    }
}

impl ros2_client::Message for NegotiationStatus {}
