use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CabinReport {
    pub header: crate::std_msgs::msg::Header,
    pub door_open_front_right: bool,
    pub door_open_front_left: bool,
    pub door_open_rear_right: bool,
    pub door_open_rear_left: bool,
    pub hood_open: bool,
    pub trunk_open: bool,
    pub passenger_present: bool,
    pub passenger_airbag_enabled: bool,
    pub seatbelt_engaged_driver: bool,
    pub seatbelt_engaged_passenger: bool,
}

impl Default for CabinReport {
    fn default() -> Self {
        CabinReport {
            header: crate::std_msgs::msg::Header::default(),
            door_open_front_right: false,
            door_open_front_left: false,
            door_open_rear_right: false,
            door_open_rear_left: false,
            hood_open: false,
            trunk_open: false,
            passenger_present: false,
            passenger_airbag_enabled: false,
            seatbelt_engaged_driver: false,
            seatbelt_engaged_passenger: false,
        }
    }
}

impl ros2_client::Message for CabinReport {}
