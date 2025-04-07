use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EsrStatus6 {
    pub header: crate::std_msgs::msg::Header,
    pub canmsg: ::std::string::String,
    pub supply_1p8v_a2d: u8,
    pub supply_n5v_a2d: u8,
    pub wave_diff_a2d: u8,
    pub sw_version_dsp_3rd_byte: u8,
    pub vertical_align_updated: bool,
    pub system_power_mode: u8,
    pub found_target: bool,
    pub recommend_unconverge: bool,
    pub factory_align_status_1: u8,
    pub factory_align_status_2: u8,
    pub factory_misalignment: f32,
    pub serv_align_updates_done: u8,
    pub vertical_misalignment: f32,
}

impl Default for EsrStatus6 {
    fn default() -> Self {
        EsrStatus6 {
            header: crate::std_msgs::msg::Header::default(),
            canmsg: ::std::string::String::new(),
            supply_1p8v_a2d: 0,
            supply_n5v_a2d: 0,
            wave_diff_a2d: 0,
            sw_version_dsp_3rd_byte: 0,
            vertical_align_updated: false,
            system_power_mode: 0,
            found_target: false,
            recommend_unconverge: false,
            factory_align_status_1: 0,
            factory_align_status_2: 0,
            factory_misalignment: 0.0,
            serv_align_updates_done: 0,
            vertical_misalignment: 0.0,
        }
    }
}

impl ros2_client::Message for EsrStatus6 {}
