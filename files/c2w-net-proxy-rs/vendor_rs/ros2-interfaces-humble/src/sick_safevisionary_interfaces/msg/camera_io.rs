use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraIO {
    pub header: crate::std_msgs::msg::Header,
    pub configured: crate::sick_safevisionary_interfaces::msg::IOConfigured,
    pub direction: crate::sick_safevisionary_interfaces::msg::IODirection,
    pub input_values: crate::sick_safevisionary_interfaces::msg::IOInputValues,
    pub output_values: crate::sick_safevisionary_interfaces::msg::IOOutputValues,
    pub ossds_state: crate::sick_safevisionary_interfaces::msg::IOOSSDSState,
    pub ossds_dyn_count: u8,
    pub ossds_crc: u8,
    pub ossds_io_status: u8,
    pub dynamic_speed_a: u16,
    pub dynamic_speed_b: u16,
    pub dynamic_valid_flags: u16,
}

impl Default for CameraIO {
    fn default() -> Self {
        CameraIO {
            header: crate::std_msgs::msg::Header::default(),
            configured: crate::sick_safevisionary_interfaces::msg::IOConfigured::default(),
            direction: crate::sick_safevisionary_interfaces::msg::IODirection::default(),
            input_values: crate::sick_safevisionary_interfaces::msg::IOInputValues::default(),
            output_values: crate::sick_safevisionary_interfaces::msg::IOOutputValues::default(),
            ossds_state: crate::sick_safevisionary_interfaces::msg::IOOSSDSState::default(),
            ossds_dyn_count: 0,
            ossds_crc: 0,
            ossds_io_status: 0,
            dynamic_speed_a: 0,
            dynamic_speed_b: 0,
            dynamic_valid_flags: 0,
        }
    }
}

impl ros2_client::Message for CameraIO {}
