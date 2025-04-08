use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EsrVehicle2 {
    pub header: crate::std_msgs::msg::Header,
    pub scan_index_ack: u16,
    pub use_angle_misalignment: bool,
    pub clear_faults: bool,
    pub high_yaw_angle: i8,
    pub mr_only_transmit: bool,
    pub lr_only_transmit: bool,
    pub angle_misalignment: f32,
    pub lateral_mounting_offset: f32,
    pub radar_cmd_radiate: bool,
    pub blockage_disable: bool,
    pub maximum_tracks: u8,
    pub turn_signal_status: u8,
    pub vehicle_speed_validity: bool,
    pub mmr_upside_down: bool,
    pub grouping_mode: u8,
    pub wiper_status: bool,
    pub raw_data_enable: bool,
}

impl Default for EsrVehicle2 {
    fn default() -> Self {
        EsrVehicle2 {
            header: crate::std_msgs::msg::Header::default(),
            scan_index_ack: 0,
            use_angle_misalignment: false,
            clear_faults: false,
            high_yaw_angle: 0,
            mr_only_transmit: false,
            lr_only_transmit: false,
            angle_misalignment: 0.0,
            lateral_mounting_offset: 0.0,
            radar_cmd_radiate: false,
            blockage_disable: false,
            maximum_tracks: 0,
            turn_signal_status: 0,
            vehicle_speed_validity: false,
            mmr_upside_down: false,
            grouping_mode: 0,
            wiper_status: false,
            raw_data_enable: false,
        }
    }
}

impl ros2_client::Message for EsrVehicle2 {}
