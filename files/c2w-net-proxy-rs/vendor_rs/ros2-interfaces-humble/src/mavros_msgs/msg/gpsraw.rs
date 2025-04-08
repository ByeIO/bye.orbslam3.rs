use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GPSRAW {
    pub header: crate::std_msgs::msg::Header,
    pub fix_type: u8,
    pub lat: i32,
    pub lon: i32,
    pub alt: i32,
    pub eph: u16,
    pub epv: u16,
    pub vel: u16,
    pub cog: u16,
    pub satellites_visible: u8,
    pub alt_ellipsoid: i32,
    pub h_acc: u32,
    pub v_acc: u32,
    pub vel_acc: u32,
    pub hdg_acc: i32,
    pub yaw: u16,
    pub dgps_numch: u8,
    pub dgps_age: u32,
}

impl GPSRAW {
    pub const GPS_FIX_TYPE_NO_GPS: u8 = 0;
    pub const GPS_FIX_TYPE_NO_FIX: u8 = 1;
    pub const GPS_FIX_TYPE_2D_FIX: u8 = 2;
    pub const GPS_FIX_TYPE_3D_FIX: u8 = 3;
    pub const GPS_FIX_TYPE_DGPS: u8 = 4;
    pub const GPS_FIX_TYPE_RTK_FLOAT: u8 = 5;
    pub const GPS_FIX_TYPE_RTK_FIXED: u8 = 6;
    pub const GPS_FIX_TYPE_STATIC: u8 = 7;
    pub const GPS_FIX_TYPE_PPP: u8 = 8;
}

impl Default for GPSRAW {
    fn default() -> Self {
        GPSRAW {
            header: crate::std_msgs::msg::Header::default(),
            fix_type: 0,
            lat: 0,
            lon: 0,
            alt: 0,
            eph: 0,
            epv: 0,
            vel: 0,
            cog: 0,
            satellites_visible: 0,
            alt_ellipsoid: 0,
            h_acc: 0,
            v_acc: 0,
            vel_acc: 0,
            hdg_acc: 0,
            yaw: 0,
            dgps_numch: 0,
            dgps_age: 0,
        }
    }
}

impl ros2_client::Message for GPSRAW {}
