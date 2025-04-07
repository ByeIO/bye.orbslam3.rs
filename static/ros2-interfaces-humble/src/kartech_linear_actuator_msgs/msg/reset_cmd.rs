use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResetCmd {
    pub header: crate::std_msgs::msg::Header,
    pub confirm: bool,
    pub reset_type: u8,
    pub reset_user_rpt_id: bool,
    pub reset_user_cmd_id_1: bool,
    pub reset_user_cmd_id_2: bool,
    pub reset_user_cmd_id_3: bool,
    pub reset_user_cmd_id_4: bool,
    pub disable_user_rpt_id: bool,
    pub reenable_default_cmd_id: bool,
}

impl ResetCmd {
    pub const RESET_OUTPUTS: u8 = 0;
    pub const RESET_USER_DEFINED_IDS: u8 = 1;
    pub const RESET_REPORT_RATES: u8 = 2;
    pub const RESET_HARDWARE_CONFIGURATIONS: u8 = 3;
    pub const RESET_USER_CONFIGURATIONS: u8 = 4;
    pub const RESET_EVERYTHING: u8 = 5;
    pub const RESET_NONE: u8 = 6;
}

impl Default for ResetCmd {
    fn default() -> Self {
        ResetCmd {
            header: crate::std_msgs::msg::Header::default(),
            confirm: false,
            reset_type: 0,
            reset_user_rpt_id: false,
            reset_user_cmd_id_1: false,
            reset_user_cmd_id_2: false,
            reset_user_cmd_id_3: false,
            reset_user_cmd_id_4: false,
            disable_user_rpt_id: false,
            reenable_default_cmd_id: false,
        }
    }
}

impl ros2_client::Message for ResetCmd {}
