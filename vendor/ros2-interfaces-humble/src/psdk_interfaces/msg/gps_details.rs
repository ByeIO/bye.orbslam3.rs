use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GPSDetails {
    pub header: crate::std_msgs::msg::Header,
    pub horizontal_dop: f32,
    pub position_dop: f32,
    pub fix_state: f32,
    pub vertical_accuracy: f32,
    pub horizontal_accuracy: f32,
    pub speed_accuracy: f32,
    pub num_gps_satellites_used: u32,
    pub num_glonass_satellites_used: u32,
    pub num_total_satellites_used: u16,
    pub gps_counter: u16,
}

impl GPSDetails {
    pub const GPS_FIX_STATE_NO_FIX: u16 = 0;
    pub const GPS_FIX_STATE_DEAD_RECKONING_ONLY: u16 = 1;
    pub const GPS_FIX_STATE_2D_FIX: u16 = 2;
    pub const GPS_FIX_STATE_3D_FIX: u16 = 3;
    pub const GPS_FIX_STATE_GPS_PLUS_DEAD_RECKONING: u16 = 4;
    pub const GPS_FIX_STATE_TIME_ONLY_FIX: u16 = 5;
}

impl Default for GPSDetails {
    fn default() -> Self {
        GPSDetails {
            header: crate::std_msgs::msg::Header::default(),
            horizontal_dop: 0.0,
            position_dop: 0.0,
            fix_state: 0.0,
            vertical_accuracy: 0.0,
            horizontal_accuracy: 0.0,
            speed_accuracy: 0.0,
            num_gps_satellites_used: 0,
            num_glonass_satellites_used: 0,
            num_total_satellites_used: 0,
            gps_counter: 0,
        }
    }
}

impl ros2_client::Message for GPSDetails {}
