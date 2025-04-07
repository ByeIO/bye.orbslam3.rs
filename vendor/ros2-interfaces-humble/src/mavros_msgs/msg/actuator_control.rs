use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActuatorControl {
    pub header: crate::std_msgs::msg::Header,
    pub group_mix: u8,
    pub controls: [f32; 8],
}

impl ActuatorControl {
    pub const PX4_MIX_FLIGHT_CONTROL: u8 = 0;
    pub const PX4_MIX_FLIGHT_CONTROL_VTOL_ALT: u8 = 1;
    pub const PX4_MIX_PAYLOAD: u8 = 2;
    pub const PX4_MIX_MANUAL_PASSTHROUGH: u8 = 3;
}

impl Default for ActuatorControl {
    fn default() -> Self {
        ActuatorControl {
            header: crate::std_msgs::msg::Header::default(),
            group_mix: 0,
            controls: [0.0; 8],
        }
    }
}

impl ros2_client::Message for ActuatorControl {}
