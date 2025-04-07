use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BatteryState {
    pub header: crate::std_msgs::msg::Header,
    pub voltage: f32,
    pub temperature: f32,
    pub current: f32,
    pub charge: f32,
    pub capacity: f32,
    pub design_capacity: f32,
    pub percentage: f32,
    pub power_supply_status: u8,
    pub power_supply_health: u8,
    pub power_supply_technology: u8,
    pub present: bool,
    pub cell_voltage: Vec<f32>,
    pub cell_temperature: Vec<f32>,
    pub location: ::std::string::String,
    pub serial_number: ::std::string::String,
}

impl BatteryState {
    pub const POWER_SUPPLY_STATUS_UNKNOWN: u8 = 0;
    pub const POWER_SUPPLY_STATUS_CHARGING: u8 = 1;
    pub const POWER_SUPPLY_STATUS_DISCHARGING: u8 = 2;
    pub const POWER_SUPPLY_STATUS_NOT_CHARGING: u8 = 3;
    pub const POWER_SUPPLY_STATUS_FULL: u8 = 4;
    pub const POWER_SUPPLY_HEALTH_UNKNOWN: u8 = 0;
    pub const POWER_SUPPLY_HEALTH_GOOD: u8 = 1;
    pub const POWER_SUPPLY_HEALTH_OVERHEAT: u8 = 2;
    pub const POWER_SUPPLY_HEALTH_DEAD: u8 = 3;
    pub const POWER_SUPPLY_HEALTH_OVERVOLTAGE: u8 = 4;
    pub const POWER_SUPPLY_HEALTH_UNSPEC_FAILURE: u8 = 5;
    pub const POWER_SUPPLY_HEALTH_COLD: u8 = 6;
    pub const POWER_SUPPLY_HEALTH_WATCHDOG_TIMER_EXPIRE: u8 = 7;
    pub const POWER_SUPPLY_HEALTH_SAFETY_TIMER_EXPIRE: u8 = 8;
    pub const POWER_SUPPLY_TECHNOLOGY_UNKNOWN: u8 = 0;
    pub const POWER_SUPPLY_TECHNOLOGY_NIMH: u8 = 1;
    pub const POWER_SUPPLY_TECHNOLOGY_LION: u8 = 2;
    pub const POWER_SUPPLY_TECHNOLOGY_LIPO: u8 = 3;
    pub const POWER_SUPPLY_TECHNOLOGY_LIFE: u8 = 4;
    pub const POWER_SUPPLY_TECHNOLOGY_NICD: u8 = 5;
    pub const POWER_SUPPLY_TECHNOLOGY_LIMN: u8 = 6;
}

impl Default for BatteryState {
    fn default() -> Self {
        BatteryState {
            header: crate::std_msgs::msg::Header::default(),
            voltage: 0.0,
            temperature: 0.0,
            current: 0.0,
            charge: 0.0,
            capacity: 0.0,
            design_capacity: 0.0,
            percentage: 0.0,
            power_supply_status: 0,
            power_supply_health: 0,
            power_supply_technology: 0,
            present: false,
            cell_voltage: Vec::new(),
            cell_temperature: Vec::new(),
            location: ::std::string::String::new(),
            serial_number: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for BatteryState {}
