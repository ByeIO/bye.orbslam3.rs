use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CanBusData {
    pub header: crate::std_msgs::msg::Header,
    pub speed_mps: f32,
    pub throttle_pct: f32,
    pub brake_pct: f32,
    pub steer_pct: f32,
    pub parking_brake_active: bool,
    pub high_beams_active: bool,
    pub low_beams_active: bool,
    pub hazard_lights_active: bool,
    pub fog_lights_active: bool,
    pub left_turn_signal_active: bool,
    pub right_turn_signal_active: bool,
    pub wipers_active: bool,
    pub reverse_gear_active: bool,
    pub selected_gear: i8,
    pub engine_active: bool,
    pub engine_rpm: f32,
    pub gps_latitude: f64,
    pub gps_longitude: f64,
    pub gps_altitude: f64,
    pub orientation: crate::geometry_msgs::msg::Quaternion,
    pub linear_velocities: crate::geometry_msgs::msg::Vector3,
}

impl CanBusData {
    pub const GEAR_NEUTRAL: i8 = 0;
    pub const GEAR_DRIVE: i8 = 1;
    pub const GEAR_REVERSE: i8 = 2;
    pub const GEAR_PARKING: i8 = 3;
    pub const GEAR_LOW: i8 = 4;
}

impl Default for CanBusData {
    fn default() -> Self {
        CanBusData {
            header: crate::std_msgs::msg::Header::default(),
            speed_mps: 0.0,
            throttle_pct: 0.0,
            brake_pct: 0.0,
            steer_pct: 0.0,
            parking_brake_active: false,
            high_beams_active: false,
            low_beams_active: false,
            hazard_lights_active: false,
            fog_lights_active: false,
            left_turn_signal_active: false,
            right_turn_signal_active: false,
            wipers_active: false,
            reverse_gear_active: false,
            selected_gear: 0,
            engine_active: false,
            engine_rpm: 0.0,
            gps_latitude: 0.0,
            gps_longitude: 0.0,
            gps_altitude: 0.0,
            orientation: crate::geometry_msgs::msg::Quaternion::default(),
            linear_velocities: crate::geometry_msgs::msg::Vector3::default(),
        }
    }
}

impl ros2_client::Message for CanBusData {}
