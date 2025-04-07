use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SystemReport {
    pub header: crate::std_msgs::msg::Header,
    pub inhibit: bool,
    pub validate_cmd_crc_rc: bool,
    pub system_sync_mode: crate::ds_dbw_msgs::msg::SystemSyncMode,
    pub state: crate::ds_dbw_msgs::msg::SystemState,
    pub reason_disengage: u8,
    pub reason_not_ready: u8,
    pub reason_disengage_str: ::std::string::String,
    pub reason_not_ready_str: ::std::string::String,
    pub btn_enable: bool,
    pub btn_disable: bool,
    pub lockout: bool,
    #[serde(rename = "override")]    pub override_: bool,
    pub ready: bool,
    pub enabled: bool,
    pub fault: bool,
    pub bad_crc: bool,
    pub bad_rc: bool,
}

impl Default for SystemReport {
    fn default() -> Self {
        SystemReport {
            header: crate::std_msgs::msg::Header::default(),
            inhibit: false,
            validate_cmd_crc_rc: false,
            system_sync_mode: crate::ds_dbw_msgs::msg::SystemSyncMode::default(),
            state: crate::ds_dbw_msgs::msg::SystemState::default(),
            reason_disengage: 0,
            reason_not_ready: 0,
            reason_disengage_str: ::std::string::String::new(),
            reason_not_ready_str: ::std::string::String::new(),
            btn_enable: false,
            btn_disable: false,
            lockout: false,
            override_: false,
            ready: false,
            enabled: false,
            fault: false,
            bad_crc: false,
            bad_rc: false,
        }
    }
}

impl ros2_client::Message for SystemReport {}
