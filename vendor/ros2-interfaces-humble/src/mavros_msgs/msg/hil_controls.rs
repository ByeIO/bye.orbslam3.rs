use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HilControls {
    pub header: crate::std_msgs::msg::Header,
    pub roll_ailerons: f32,
    pub pitch_elevator: f32,
    pub yaw_rudder: f32,
    pub throttle: f32,
    pub aux1: f32,
    pub aux2: f32,
    pub aux3: f32,
    pub aux4: f32,
    pub mode: u8,
    pub nav_mode: u8,
}

impl Default for HilControls {
    fn default() -> Self {
        HilControls {
            header: crate::std_msgs::msg::Header::default(),
            roll_ailerons: 0.0,
            pitch_elevator: 0.0,
            yaw_rudder: 0.0,
            throttle: 0.0,
            aux1: 0.0,
            aux2: 0.0,
            aux3: 0.0,
            aux4: 0.0,
            mode: 0,
            nav_mode: 0,
        }
    }
}

impl ros2_client::Message for HilControls {}
