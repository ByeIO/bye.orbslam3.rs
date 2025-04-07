use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EstimatorStatus {
    pub header: crate::std_msgs::msg::Header,
    pub attitude_status_flag: bool,
    pub velocity_horiz_status_flag: bool,
    pub velocity_vert_status_flag: bool,
    pub pos_horiz_rel_status_flag: bool,
    pub pos_horiz_abs_status_flag: bool,
    pub pos_vert_abs_status_flag: bool,
    pub pos_vert_agl_status_flag: bool,
    pub const_pos_mode_status_flag: bool,
    pub pred_pos_horiz_rel_status_flag: bool,
    pub pred_pos_horiz_abs_status_flag: bool,
    pub gps_glitch_status_flag: bool,
    pub accel_error_status_flag: bool,
}

impl Default for EstimatorStatus {
    fn default() -> Self {
        EstimatorStatus {
            header: crate::std_msgs::msg::Header::default(),
            attitude_status_flag: false,
            velocity_horiz_status_flag: false,
            velocity_vert_status_flag: false,
            pos_horiz_rel_status_flag: false,
            pos_horiz_abs_status_flag: false,
            pos_vert_abs_status_flag: false,
            pos_vert_agl_status_flag: false,
            const_pos_mode_status_flag: false,
            pred_pos_horiz_rel_status_flag: false,
            pred_pos_horiz_abs_status_flag: false,
            gps_glitch_status_flag: false,
            accel_error_status_flag: false,
        }
    }
}

impl ros2_client::Message for EstimatorStatus {}
