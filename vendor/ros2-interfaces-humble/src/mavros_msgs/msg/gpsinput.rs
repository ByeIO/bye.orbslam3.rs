use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GPSINPUT {
    pub header: crate::std_msgs::msg::Header,
    pub fix_type: u8,
    pub gps_id: u8,
    pub ignore_flags: u16,
    pub time_week_ms: u32,
    pub time_week: u16,
    pub lat: i32,
    pub lon: i32,
    pub alt: f32,
    pub hdop: f32,
    pub vdop: f32,
    pub vn: f32,
    pub ve: f32,
    pub vd: f32,
    pub speed_accuracy: f32,
    pub horiz_accuracy: f32,
    pub vert_accuracy: f32,
    pub satellites_visible: u8,
    pub yaw: u16,
}

impl GPSINPUT {
    pub const GPS_FIX_TYPE_NO_GPS: u8 = 0;
    pub const GPS_FIX_TYPE_NO_FIX: u8 = 1;
    pub const GPS_FIX_TYPE_2D_FIX: u8 = 2;
    pub const GPS_FIX_TYPE_3D_FIX: u8 = 3;
    pub const GPS_FIX_TYPE_DGPS: u8 = 4;
    pub const GPS_FIX_TYPE_RTK_FLOATR: u8 = 5;
    pub const GPS_FIX_TYPE_RTK_FIXEDR: u8 = 6;
    pub const GPS_FIX_TYPE_STATIC: u8 = 7;
    pub const GPS_FIX_TYPE_PPP: u8 = 8;
}

impl Default for GPSINPUT {
    fn default() -> Self {
        GPSINPUT {
            header: crate::std_msgs::msg::Header::default(),
            fix_type: 0,
            gps_id: 0,
            ignore_flags: 0,
            time_week_ms: 0,
            time_week: 0,
            lat: 0,
            lon: 0,
            alt: 0.0,
            hdop: 0.0,
            vdop: 0.0,
            vn: 0.0,
            ve: 0.0,
            vd: 0.0,
            speed_accuracy: 0.0,
            horiz_accuracy: 0.0,
            vert_accuracy: 0.0,
            satellites_visible: 0,
            yaw: 0,
        }
    }
}

impl ros2_client::Message for GPSINPUT {}
