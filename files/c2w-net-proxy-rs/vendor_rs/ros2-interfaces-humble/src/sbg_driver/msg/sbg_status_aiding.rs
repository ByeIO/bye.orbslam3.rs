use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SbgStatusAiding {
    pub gps1_pos_recv: bool,
    pub gps1_vel_recv: bool,
    pub gps1_hdt_recv: bool,
    pub gps1_utc_recv: bool,
    pub gps2_pos_recv: bool,
    pub gps2_vel_recv: bool,
    pub gps2_hdt_recv: bool,
    pub gps2_utc_recv: bool,
    pub mag_recv: bool,
    pub odo_recv: bool,
    pub dvl_recv: bool,
    pub usbl_recv: bool,
    pub depth_recv: bool,
    pub air_data_recv: bool,
    pub user_pos_recv: bool,
    pub user_vel_recv: bool,
    pub user_heading_recv: bool,
}

impl Default for SbgStatusAiding {
    fn default() -> Self {
        SbgStatusAiding {
            gps1_pos_recv: false,
            gps1_vel_recv: false,
            gps1_hdt_recv: false,
            gps1_utc_recv: false,
            gps2_pos_recv: false,
            gps2_vel_recv: false,
            gps2_hdt_recv: false,
            gps2_utc_recv: false,
            mag_recv: false,
            odo_recv: false,
            dvl_recv: false,
            usbl_recv: false,
            depth_recv: false,
            air_data_recv: false,
            user_pos_recv: false,
            user_vel_recv: false,
            user_heading_recv: false,
        }
    }
}

impl ros2_client::Message for SbgStatusAiding {}
