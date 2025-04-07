use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FlightStatus {
    pub header: crate::std_msgs::msg::Header,
    pub flight_status: u8,
}

impl FlightStatus {
    pub const FLIGHT_STATUS_STOPED: u8 = 0;
    pub const FLIGHT_STATUS_ON_GROUND: u8 = 1;
    pub const FLIGHT_STATUS_ON_AIR: u8 = 2;
}

impl Default for FlightStatus {
    fn default() -> Self {
        FlightStatus {
            header: crate::std_msgs::msg::Header::default(),
            flight_status: 0,
        }
    }
}

impl ros2_client::Message for FlightStatus {}
