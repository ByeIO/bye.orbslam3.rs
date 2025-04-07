use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Power {
    pub header: crate::std_msgs::msg::Header,
    pub shore_power_connected: i8,
    pub battery_connected: i8,
    pub power_12v_user_nominal: i8,
    pub charger_connected: i8,
    pub charging_complete: i8,
    pub measured_voltages: Vec<f32>,
    pub measured_currents: Vec<f32>,
}

impl Power {
    pub const NOT_APPLICABLE: i8 = -1;
    pub const J100_MEASURED_BATTERY: u8 = 0;
    pub const J100_MEASURED_5V: u8 = 1;
    pub const J100_MEASURED_12V: u8 = 2;
    pub const D100_MEASURED_BATTERY: u8 = 0;
    pub const D100_MEASURED_5V: u8 = 1;
    pub const D100_MEASURED_12V: u8 = 2;
    pub const D150_MEASURED_BATTERY: u8 = 0;
    pub const D150_MEASURED_5V: u8 = 1;
    pub const D150_MEASURED_12V: u8 = 2;
    pub const W200_MEASURED_BATTERY: u8 = 0;
    pub const W200_MEASURED_12V: u8 = 1;
    pub const W200_MEASURED_24V: u8 = 2;
    pub const W200_MEASURED_48V: u8 = 3;
    pub const R100_MEASURED_BATTERY: u8 = 0;
    pub const R100_MEASURED_5V: u8 = 1;
    pub const R100_MEASURED_12V: u8 = 2;
    pub const R100_MEASURED_INVERTER: u8 = 3;
    pub const R100_MEASURED_FRONT_AXLE: u8 = 4;
    pub const R100_MEASURED_REAR_AXLE: u8 = 5;
    pub const R100_MEASURED_LIGHT: u8 = 6;
    pub const A200_BATTERY_VOLTAGE: u8 = 0;
    pub const A200_LEFT_DRIVER_VOLTAGE: u8 = 1;
    pub const A200_RIGHT_DRIVER_VOLTAGE: u8 = 2;
    pub const A200_VOLTAGES_SIZE: u8 = 3;
    pub const J100_TOTAL_CURRENT: u8 = 0;
    pub const J100_COMPUTER_CURRENT: u8 = 1;
    pub const J100_DRIVE_CURRENT: u8 = 2;
    pub const J100_USER_CURRENT: u8 = 3;
    pub const D100_TOTAL_CURRENT: u8 = 0;
    pub const D100_COMPUTER_CURRENT: u8 = 1;
    pub const D150_TOTAL_CURRENT: u8 = 0;
    pub const D150_COMPUTER_CURRENT: u8 = 1;
    pub const W200_TOTAL_CURRENT: u8 = 0;
    pub const W200_COMPUTER_CURRENT: u8 = 1;
    pub const W200_12V_CURRENT: u8 = 2;
    pub const W200_24V_CURRENT: u8 = 3;
    pub const R100_TOTAL_CURRENT: u8 = 0;
    pub const A200_MCU_AND_USER_PORT_CURRENT: u8 = 0;
    pub const A200_LEFT_DRIVER_CURRENT: u8 = 1;
    pub const A200_RIGHT_DRIVER_CURRENT: u8 = 2;
    pub const A200_CURRENTS_SIZE: u8 = 3;
}

impl Default for Power {
    fn default() -> Self {
        Power {
            header: crate::std_msgs::msg::Header::default(),
            shore_power_connected: 0,
            battery_connected: 0,
            power_12v_user_nominal: 0,
            charger_connected: 0,
            charging_complete: 0,
            measured_voltages: Vec::new(),
            measured_currents: Vec::new(),
        }
    }
}

impl ros2_client::Message for Power {}
