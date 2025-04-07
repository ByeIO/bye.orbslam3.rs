use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Status {
    pub header: crate::std_msgs::msg::Header,
    pub hardware_id: ::std::string::String,
    pub firmware_version: ::std::string::String,
    pub mcu_uptime: crate::builtin_interfaces::msg::Duration,
    pub connection_uptime: crate::builtin_interfaces::msg::Duration,
    pub pcb_temperature: f32,
    pub mcu_temperature: f32,
}

impl Default for Status {
    fn default() -> Self {
        Status {
            header: crate::std_msgs::msg::Header::default(),
            hardware_id: ::std::string::String::new(),
            firmware_version: ::std::string::String::new(),
            mcu_uptime: crate::builtin_interfaces::msg::Duration::default(),
            connection_uptime: crate::builtin_interfaces::msg::Duration::default(),
            pcb_temperature: 0.0,
            mcu_temperature: 0.0,
        }
    }
}

impl ros2_client::Message for Status {}
