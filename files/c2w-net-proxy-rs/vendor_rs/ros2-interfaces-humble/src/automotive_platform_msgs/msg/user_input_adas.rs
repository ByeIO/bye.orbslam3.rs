use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserInputADAS {
    pub header: crate::std_msgs::msg::Header,
    pub btn_cc_on: bool,
    pub btn_cc_off: bool,
    pub btn_cc_on_off: bool,
    pub btn_cc_set_inc: bool,
    pub btn_cc_set_dec: bool,
    pub btn_cc_res: bool,
    pub btn_cc_cncl: bool,
    pub btn_cc_res_cncl: bool,
    pub btn_acc_gap_inc: bool,
    pub btn_acc_gap_dec: bool,
    pub btn_lka_on: bool,
    pub btn_lka_off: bool,
    pub btn_lka_on_off: bool,
}

impl Default for UserInputADAS {
    fn default() -> Self {
        UserInputADAS {
            header: crate::std_msgs::msg::Header::default(),
            btn_cc_on: false,
            btn_cc_off: false,
            btn_cc_on_off: false,
            btn_cc_set_inc: false,
            btn_cc_set_dec: false,
            btn_cc_res: false,
            btn_cc_cncl: false,
            btn_cc_res_cncl: false,
            btn_acc_gap_inc: false,
            btn_acc_gap_dec: false,
            btn_lka_on: false,
            btn_lka_off: false,
            btn_lka_on_off: false,
        }
    }
}

impl ros2_client::Message for UserInputADAS {}
