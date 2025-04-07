use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RadarSector {
    pub header: crate::std_msgs::msg::Header,
    pub angle_start: f32,
    pub angle_increment: f32,
    pub time_increment: crate::builtin_interfaces::msg::Duration,
    pub scan_time: crate::builtin_interfaces::msg::Duration,
    pub range_min: f32,
    pub range_max: f32,
    pub intensities: Vec<crate::marine_sensor_msgs::msg::RadarEcho>,
}

impl Default for RadarSector {
    fn default() -> Self {
        RadarSector {
            header: crate::std_msgs::msg::Header::default(),
            angle_start: 0.0,
            angle_increment: 0.0,
            time_increment: crate::builtin_interfaces::msg::Duration::default(),
            scan_time: crate::builtin_interfaces::msg::Duration::default(),
            range_min: 0.0,
            range_max: 0.0,
            intensities: Vec::new(),
        }
    }
}

impl ros2_client::Message for RadarSector {}
