use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MrrDetection {
    pub header: crate::std_msgs::msg::Header,
    pub detection_id: u8,
    pub confid_azimuth: u8,
    pub super_res_target: bool,
    pub nd_target: bool,
    pub host_veh_clutter: bool,
    pub valid_level: bool,
    pub azimuth: f32,
    pub range: f32,
    pub range_rate: f32,
    pub amplitude: i8,
    pub index_2lsb: u8,
}

impl Default for MrrDetection {
    fn default() -> Self {
        MrrDetection {
            header: crate::std_msgs::msg::Header::default(),
            detection_id: 0,
            confid_azimuth: 0,
            super_res_target: false,
            nd_target: false,
            host_veh_clutter: false,
            valid_level: false,
            azimuth: 0.0,
            range: 0.0,
            range_rate: 0.0,
            amplitude: 0,
            index_2lsb: 0,
        }
    }
}

impl ros2_client::Message for MrrDetection {}
