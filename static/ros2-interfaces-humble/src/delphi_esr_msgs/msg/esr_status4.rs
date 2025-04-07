use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EsrStatus4 {
    pub header: crate::std_msgs::msg::Header,
    pub canmsg: ::std::string::String,
    pub truck_target_det: bool,
    pub lr_only_grating_lobe_det: bool,
    pub sidelobe_blockage: bool,
    pub partial_blockage: bool,
    pub mr_lr_mode: u8,
    pub rolling_count_3: u8,
    pub path_id_acc: u8,
    pub path_id_cmbb_move: u8,
    pub path_id_cmbb_stat: u8,
    pub path_id_fcw_move: u8,
    pub path_id_fcw_stat: u8,
    pub auto_align_angle: f32,
    pub path_id_acc_stat: u8,
}

impl Default for EsrStatus4 {
    fn default() -> Self {
        EsrStatus4 {
            header: crate::std_msgs::msg::Header::default(),
            canmsg: ::std::string::String::new(),
            truck_target_det: false,
            lr_only_grating_lobe_det: false,
            sidelobe_blockage: false,
            partial_blockage: false,
            mr_lr_mode: 0,
            rolling_count_3: 0,
            path_id_acc: 0,
            path_id_cmbb_move: 0,
            path_id_cmbb_stat: 0,
            path_id_fcw_move: 0,
            path_id_fcw_stat: 0,
            auto_align_angle: 0.0,
            path_id_acc_stat: 0,
        }
    }
}

impl ros2_client::Message for EsrStatus4 {}
