use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ADSBVehicle {
    pub header: crate::std_msgs::msg::Header,
    pub icao_address: u32,
    pub callsign: ::std::string::String,
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f32,
    pub heading: f32,
    pub hor_velocity: f32,
    pub ver_velocity: f32,
    pub altitude_type: u8,
    pub emitter_type: u8,
    pub tslc: crate::builtin_interfaces::msg::Duration,
    pub flags: u16,
    pub squawk: u16,
}

impl ADSBVehicle {
    pub const ALT_PRESSURE_QNH: u8 = 0;
    pub const ALT_GEOMETRIC: u8 = 1;
    pub const EMITTER_NO_INFO: u8 = 0;
    pub const EMITTER_LIGHT: u8 = 1;
    pub const EMITTER_SMALL: u8 = 2;
    pub const EMITTER_LARGE: u8 = 3;
    pub const EMITTER_HIGH_VORTEX_LARGE: u8 = 4;
    pub const EMITTER_HEAVY: u8 = 5;
    pub const EMITTER_HIGHLY_MANUV: u8 = 6;
    pub const EMITTER_ROTOCRAFT: u8 = 7;
    pub const EMITTER_UNASSIGNED: u8 = 8;
    pub const EMITTER_GLIDER: u8 = 9;
    pub const EMITTER_LIGHTER_AIR: u8 = 10;
    pub const EMITTER_PARACHUTE: u8 = 11;
    pub const EMITTER_ULTRA_LIGHT: u8 = 12;
    pub const EMITTER_UNASSIGNED2: u8 = 13;
    pub const EMITTER_UAV: u8 = 14;
    pub const EMITTER_SPACE: u8 = 15;
    pub const EMITTER_UNASSGINED3: u8 = 16;
    pub const EMITTER_EMERGENCY_SURFACE: u8 = 17;
    pub const EMITTER_SERVICE_SURFACE: u8 = 18;
    pub const EMITTER_POINT_OBSTACLE: u8 = 19;
    pub const FLAG_VALID_COORDS: u16 = 1;
    pub const FLAG_VALID_ALTITUDE: u16 = 2;
    pub const FLAG_VALID_HEADING: u16 = 4;
    pub const FLAG_VALID_VELOCITY: u16 = 8;
    pub const FLAG_VALID_CALLSIGN: u16 = 16;
    pub const FLAG_VALID_SQUAWK: u16 = 32;
    pub const FLAG_SIMULATED: u16 = 64;
    pub const FLAG_VERTICAL_VELOCITY_VALID: u16 = 128;
    pub const FLAG_BARO_VALID: u16 = 256;
    pub const FLAG_SOURCE_UAT: u16 = 32768;
}

impl Default for ADSBVehicle {
    fn default() -> Self {
        ADSBVehicle {
            header: crate::std_msgs::msg::Header::default(),
            icao_address: 0,
            callsign: ::std::string::String::new(),
            latitude: 0.0,
            longitude: 0.0,
            altitude: 0.0,
            heading: 0.0,
            hor_velocity: 0.0,
            ver_velocity: 0.0,
            altitude_type: 0,
            emitter_type: 0,
            tslc: crate::builtin_interfaces::msg::Duration::default(),
            flags: 0,
            squawk: 0,
        }
    }
}

impl ros2_client::Message for ADSBVehicle {}
