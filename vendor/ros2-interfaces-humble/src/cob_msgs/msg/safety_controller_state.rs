use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SafetyControllerState {
    pub header: crate::std_msgs::msg::Header,
    pub has_wireless_emstop: bool,
    pub has_fall_sensors: bool,
    pub has_magnetic_safety_switch: bool,
    pub ack_needed: bool,
    pub emergency_button_stop: bool,
    pub brake_button_stop: bool,
    pub laser_stop: bool,
    pub wireless_stop: bool,
    pub fall_sensor_stop: bool,
    pub external_stop: bool,
    pub laser_bridged: bool,
    pub wireless_bridged: bool,
    pub magnetic_safety_switch: bool,
    pub laser_front_ok: bool,
    pub laser_left_ok: bool,
    pub laser_right_ok: bool,
    pub fall_sensor_front: bool,
    pub fall_sensor_left: bool,
    pub fall_sensor_right: bool,
    pub fall_sensor_released: bool,
    pub base_active: bool,
    pub torso_active: bool,
    pub base_enabled: bool,
    pub torso_enabled: bool,
}

impl Default for SafetyControllerState {
    fn default() -> Self {
        SafetyControllerState {
            header: crate::std_msgs::msg::Header::default(),
            has_wireless_emstop: false,
            has_fall_sensors: false,
            has_magnetic_safety_switch: false,
            ack_needed: false,
            emergency_button_stop: false,
            brake_button_stop: false,
            laser_stop: false,
            wireless_stop: false,
            fall_sensor_stop: false,
            external_stop: false,
            laser_bridged: false,
            wireless_bridged: false,
            magnetic_safety_switch: false,
            laser_front_ok: false,
            laser_left_ok: false,
            laser_right_ok: false,
            fall_sensor_front: false,
            fall_sensor_left: false,
            fall_sensor_right: false,
            fall_sensor_released: false,
            base_active: false,
            torso_active: false,
            base_enabled: false,
            torso_enabled: false,
        }
    }
}

impl ros2_client::Message for SafetyControllerState {}
