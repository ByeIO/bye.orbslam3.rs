use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GPSStatus {
    pub header: crate::std_msgs::msg::Header,
    pub satellites_used: u16,
    pub satellite_used_prn: Vec<i32>,
    pub satellites_visible: u16,
    pub satellite_visible_prn: Vec<i32>,
    pub satellite_visible_z: Vec<i32>,
    pub satellite_visible_azimuth: Vec<i32>,
    pub satellite_visible_snr: Vec<i32>,
    pub status: i16,
    pub motion_source: u16,
    pub orientation_source: u16,
    pub position_source: u16,
}

impl GPSStatus {
    pub const STATUS_NO_FIX: i16 = -1;
    pub const STATUS_FIX: i16 = 0;
    pub const STATUS_SBAS_FIX: i16 = 1;
    pub const STATUS_GBAS_FIX: i16 = 2;
    pub const STATUS_DGPS_FIX: i16 = 18;
    pub const STATUS_RTK_FIX: i16 = 19;
    pub const STATUS_RTK_FLOAT: i16 = 20;
    pub const STATUS_WAAS_FIX: i16 = 33;
    pub const SOURCE_NONE: u16 = 0;
    pub const SOURCE_GPS: u16 = 1;
    pub const SOURCE_POINTS: u16 = 2;
    pub const SOURCE_DOPPLER: u16 = 4;
    pub const SOURCE_ALTIMETER: u16 = 8;
    pub const SOURCE_MAGNETIC: u16 = 16;
    pub const SOURCE_GYRO: u16 = 32;
    pub const SOURCE_ACCEL: u16 = 64;
}

impl Default for GPSStatus {
    fn default() -> Self {
        GPSStatus {
            header: crate::std_msgs::msg::Header::default(),
            satellites_used: 0,
            satellite_used_prn: Vec::new(),
            satellites_visible: 0,
            satellite_visible_prn: Vec::new(),
            satellite_visible_z: Vec::new(),
            satellite_visible_azimuth: Vec::new(),
            satellite_visible_snr: Vec::new(),
            status: 0,
            motion_source: 0,
            orientation_source: 0,
            position_source: 0,
        }
    }
}

impl ros2_client::Message for GPSStatus {}
