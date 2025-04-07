use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChargingAssignment {
    pub robot_name: ::std::string::String,
    pub waypoint_name: ::std::string::String,
    pub mode: u8,
}

impl ChargingAssignment {
    pub const MODE_CHARGE: u8 = 0;
    pub const MODE_WAIT: u8 = 1;
}

impl Default for ChargingAssignment {
    fn default() -> Self {
        ChargingAssignment {
            robot_name: ::std::string::String::new(),
            waypoint_name: ::std::string::String::new(),
            mode: 0,
        }
    }
}

impl ros2_client::Message for ChargingAssignment {}
