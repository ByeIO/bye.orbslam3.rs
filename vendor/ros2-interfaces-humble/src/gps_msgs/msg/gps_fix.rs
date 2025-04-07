use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GPSFix {
    pub header: crate::std_msgs::msg::Header,
    pub status: crate::gps_msgs::msg::GPSStatus,
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
    pub track: f64,
    pub speed: f64,
    pub climb: f64,
    pub pitch: f64,
    pub roll: f64,
    pub dip: f64,
    pub time: f64,
    pub gdop: f64,
    pub pdop: f64,
    pub hdop: f64,
    pub vdop: f64,
    pub tdop: f64,
    pub err: f64,
    pub err_horz: f64,
    pub err_vert: f64,
    pub err_track: f64,
    pub err_speed: f64,
    pub err_climb: f64,
    pub err_time: f64,
    pub err_pitch: f64,
    pub err_roll: f64,
    pub err_dip: f64,
    pub position_covariance: [f64; 9],
    pub position_covariance_type: u8,
}

impl GPSFix {
    pub const COVARIANCE_TYPE_UNKNOWN: u8 = 0;
    pub const COVARIANCE_TYPE_APPROXIMATED: u8 = 1;
    pub const COVARIANCE_TYPE_DIAGONAL_KNOWN: u8 = 2;
    pub const COVARIANCE_TYPE_KNOWN: u8 = 3;
}

impl Default for GPSFix {
    fn default() -> Self {
        GPSFix {
            header: crate::std_msgs::msg::Header::default(),
            status: crate::gps_msgs::msg::GPSStatus::default(),
            latitude: 0.0,
            longitude: 0.0,
            altitude: 0.0,
            track: 0.0,
            speed: 0.0,
            climb: 0.0,
            pitch: 0.0,
            roll: 0.0,
            dip: 0.0,
            time: 0.0,
            gdop: 0.0,
            pdop: 0.0,
            hdop: 0.0,
            vdop: 0.0,
            tdop: 0.0,
            err: 0.0,
            err_horz: 0.0,
            err_vert: 0.0,
            err_track: 0.0,
            err_speed: 0.0,
            err_climb: 0.0,
            err_time: 0.0,
            err_pitch: 0.0,
            err_roll: 0.0,
            err_dip: 0.0,
            position_covariance: [0.0; 9],
            position_covariance_type: 0,
        }
    }
}

impl ros2_client::Message for GPSFix {}
