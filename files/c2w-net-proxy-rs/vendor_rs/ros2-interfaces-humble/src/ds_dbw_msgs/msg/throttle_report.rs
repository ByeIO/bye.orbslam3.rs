use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThrottleReport {
    pub header: crate::std_msgs::msg::Header,
    pub cmd_type: u8,
    pub percent_input: f32,
    pub percent_cmd: f32,
    pub percent_output: f32,
    pub yield_request: bool,
    pub limiting_value: bool,
    pub limiting_rate: bool,
    pub external_control: bool,
    pub ready: bool,
    pub enabled: bool,
    pub override_active: bool,
    pub override_other: bool,
    pub override_latched: bool,
    pub timeout: bool,
    pub fault: bool,
    pub bad_crc: bool,
    pub bad_rc: bool,
    pub degraded: bool,
    pub limit_value: f32,
    pub cmd_src: crate::ds_dbw_msgs::msg::CmdSrc,
}

impl Default for ThrottleReport {
    fn default() -> Self {
        ThrottleReport {
            header: crate::std_msgs::msg::Header::default(),
            cmd_type: 0,
            percent_input: 0.0,
            percent_cmd: 0.0,
            percent_output: 0.0,
            yield_request: false,
            limiting_value: false,
            limiting_rate: false,
            external_control: false,
            ready: false,
            enabled: false,
            override_active: false,
            override_other: false,
            override_latched: false,
            timeout: false,
            fault: false,
            bad_crc: false,
            bad_rc: false,
            degraded: false,
            limit_value: 0.0,
            cmd_src: crate::ds_dbw_msgs::msg::CmdSrc::default(),
        }
    }
}

impl ros2_client::Message for ThrottleReport {}
