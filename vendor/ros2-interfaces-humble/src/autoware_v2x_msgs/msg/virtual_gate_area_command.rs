use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VirtualGateAreaCommand {
    pub command: u16,
    pub sequence_id: u16,
    pub area_id: ::std::string::String,
    pub gate_ids: Vec<::std::string::String>,
    pub expected_time_arrival: Vec<crate::builtin_interfaces::msg::Time>,
}

impl VirtualGateAreaCommand {
    pub const ACQUIRE: u16 = 1;
    pub const RELEASE: u16 = 2;
}

impl Default for VirtualGateAreaCommand {
    fn default() -> Self {
        VirtualGateAreaCommand {
            command: 0,
            sequence_id: 0,
            area_id: ::std::string::String::new(),
            gate_ids: Vec::new(),
            expected_time_arrival: Vec::new(),
        }
    }
}

impl ros2_client::Message for VirtualGateAreaCommand {}
