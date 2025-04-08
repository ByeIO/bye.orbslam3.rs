use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SbgGpsPos {
    pub header: crate::std_msgs::msg::Header,
    pub time_stamp: u32,
    pub status: crate::sbg_driver::msg::SbgGpsPosStatus,
    pub gps_tow: u32,
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
    pub undulation: f32,
    pub position_accuracy: crate::geometry_msgs::msg::Vector3,
    pub num_sv_tracked: u8,
    pub num_sv_used: u8,
    pub base_station_id: u16,
    pub diff_age: u16,
}

impl Default for SbgGpsPos {
    fn default() -> Self {
        SbgGpsPos {
            header: crate::std_msgs::msg::Header::default(),
            time_stamp: 0,
            status: crate::sbg_driver::msg::SbgGpsPosStatus::default(),
            gps_tow: 0,
            latitude: 0.0,
            longitude: 0.0,
            altitude: 0.0,
            undulation: 0.0,
            position_accuracy: crate::geometry_msgs::msg::Vector3::default(),
            num_sv_tracked: 0,
            num_sv_used: 0,
            base_station_id: 0,
            diff_age: 0,
        }
    }
}

impl ros2_client::Message for SbgGpsPos {}
