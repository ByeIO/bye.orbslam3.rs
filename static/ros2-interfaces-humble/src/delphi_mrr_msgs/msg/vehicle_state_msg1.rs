use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VehicleStateMsg1 {
    pub header: crate::std_msgs::msg::Header,
    pub can_fcw_sensitivity_level: u8,
    pub can_vehicle_stationary: bool,
    pub can_intf_minor_version: u8,
    pub can_intf_major_version: u8,
    pub can_brake_pedal: u8,
    pub can_high_wheel_slip: bool,
    pub can_turn_signal_status: u8,
    pub can_washer_front_cmd: bool,
    pub can_wiper_front_cmd: bool,
    pub can_wiper_speed_info: u8,
    pub can_reverse_gear: bool,
    pub can_beam_shape_actual_right: u8,
    pub can_beam_shape_actual_left: u8,
    pub can_main_beam_indication: bool,
    pub can_vehicle_index: u16,
}

impl Default for VehicleStateMsg1 {
    fn default() -> Self {
        VehicleStateMsg1 {
            header: crate::std_msgs::msg::Header::default(),
            can_fcw_sensitivity_level: 0,
            can_vehicle_stationary: false,
            can_intf_minor_version: 0,
            can_intf_major_version: 0,
            can_brake_pedal: 0,
            can_high_wheel_slip: false,
            can_turn_signal_status: 0,
            can_washer_front_cmd: false,
            can_wiper_front_cmd: false,
            can_wiper_speed_info: 0,
            can_reverse_gear: false,
            can_beam_shape_actual_right: 0,
            can_beam_shape_actual_left: 0,
            can_main_beam_indication: false,
            can_vehicle_index: 0,
        }
    }
}

impl ros2_client::Message for VehicleStateMsg1 {}
