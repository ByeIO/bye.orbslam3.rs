use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SbgEkfStatus {
    pub solution_mode: u8,
    pub attitude_valid: bool,
    pub heading_valid: bool,
    pub velocity_valid: bool,
    pub position_valid: bool,
    pub vert_ref_used: bool,
    pub mag_ref_used: bool,
    pub gps1_vel_used: bool,
    pub gps1_pos_used: bool,
    pub gps1_hdt_used: bool,
    pub gps2_vel_used: bool,
    pub gps2_pos_used: bool,
    pub gps2_hdt_used: bool,
    pub odo_used: bool,
    pub dvl_bt_used: bool,
    pub dvl_wt_used: bool,
    pub user_pos_used: bool,
    pub user_vel_used: bool,
    pub user_heading_used: bool,
    pub usbl_used: bool,
    pub air_data_used: bool,
    pub zupt_used: bool,
    pub align_valid: bool,
    pub depth_used: bool,
}

impl Default for SbgEkfStatus {
    fn default() -> Self {
        SbgEkfStatus {
            solution_mode: 0,
            attitude_valid: false,
            heading_valid: false,
            velocity_valid: false,
            position_valid: false,
            vert_ref_used: false,
            mag_ref_used: false,
            gps1_vel_used: false,
            gps1_pos_used: false,
            gps1_hdt_used: false,
            gps2_vel_used: false,
            gps2_pos_used: false,
            gps2_hdt_used: false,
            odo_used: false,
            dvl_bt_used: false,
            dvl_wt_used: false,
            user_pos_used: false,
            user_vel_used: false,
            user_heading_used: false,
            usbl_used: false,
            air_data_used: false,
            zupt_used: false,
            align_valid: false,
            depth_used: false,
        }
    }
}

impl ros2_client::Message for SbgEkfStatus {}
