use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserInputMenus {
    pub header: crate::std_msgs::msg::Header,
    pub str_whl_left_btn_left: bool,
    pub str_whl_left_btn_down: bool,
    pub str_whl_left_btn_right: bool,
    pub str_whl_left_btn_up: bool,
    pub str_whl_left_btn_ok: bool,
    pub str_whl_right_btn_left: bool,
    pub str_whl_right_btn_down: bool,
    pub str_whl_right_btn_right: bool,
    pub str_whl_right_btn_up: bool,
    pub str_whl_right_btn_ok: bool,
    pub cntr_cons_btn_left: bool,
    pub cntr_cons_btn_down: bool,
    pub cntr_cons_btn_right: bool,
    pub cntr_cons_btn_up: bool,
    pub cntr_cons_btn_ok: bool,
}

impl Default for UserInputMenus {
    fn default() -> Self {
        UserInputMenus {
            header: crate::std_msgs::msg::Header::default(),
            str_whl_left_btn_left: false,
            str_whl_left_btn_down: false,
            str_whl_left_btn_right: false,
            str_whl_left_btn_up: false,
            str_whl_left_btn_ok: false,
            str_whl_right_btn_left: false,
            str_whl_right_btn_down: false,
            str_whl_right_btn_right: false,
            str_whl_right_btn_up: false,
            str_whl_right_btn_ok: false,
            cntr_cons_btn_left: false,
            cntr_cons_btn_down: false,
            cntr_cons_btn_right: false,
            cntr_cons_btn_up: false,
            cntr_cons_btn_ok: false,
        }
    }
}

impl ros2_client::Message for UserInputMenus {}
