use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VirtualGateAreaStatus {
    pub stamp: crate::builtin_interfaces::msg::Time,
    pub status: u16,
    pub sequence_id: u16,
    pub area_id: ::std::string::String,
    pub gate_ids: Vec<::std::string::String>,
    pub expected_time_arrival: Vec<crate::builtin_interfaces::msg::Time>,
}

impl VirtualGateAreaStatus {
    pub const RESERVED: u16 = 1;
    pub const ACQUIRED: u16 = 2;
    pub const RELEASED: u16 = 3;
}

impl Default for VirtualGateAreaStatus {
    fn default() -> Self {
        VirtualGateAreaStatus {
            stamp: crate::builtin_interfaces::msg::Time::default(),
            status: 0,
            sequence_id: 0,
            area_id: ::std::string::String::new(),
            gate_ids: Vec::new(),
            expected_time_arrival: Vec::new(),
        }
    }
}

impl ros2_client::Message for VirtualGateAreaStatus {}
