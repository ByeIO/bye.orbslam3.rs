use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Insstdev {
    pub header: crate::std_msgs::msg::Header,
    pub novatel_msg_header: crate::novatel_gps_msgs::msg::NovatelMessageHeader,
    pub latitude_dev: f32,
    pub longitude_dev: f32,
    pub height_dev: f32,
    pub north_velocity_dev: f32,
    pub east_velocity_dev: f32,
    pub up_velocity_dev: f32,
    pub roll_dev: f32,
    pub pitch_dev: f32,
    pub azimuth_dev: f32,
    pub extended_solution_status: crate::novatel_gps_msgs::msg::NovatelExtendedSolutionStatus,
    pub time_since_update: u16,
}

impl Default for Insstdev {
    fn default() -> Self {
        Insstdev {
            header: crate::std_msgs::msg::Header::default(),
            novatel_msg_header: crate::novatel_gps_msgs::msg::NovatelMessageHeader::default(),
            latitude_dev: 0.0,
            longitude_dev: 0.0,
            height_dev: 0.0,
            north_velocity_dev: 0.0,
            east_velocity_dev: 0.0,
            up_velocity_dev: 0.0,
            roll_dev: 0.0,
            pitch_dev: 0.0,
            azimuth_dev: 0.0,
            extended_solution_status: crate::novatel_gps_msgs::msg::NovatelExtendedSolutionStatus::default(),
            time_since_update: 0,
        }
    }
}

impl ros2_client::Message for Insstdev {}
