use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HilActuatorControls {
    pub header: crate::std_msgs::msg::Header,
    pub controls: [f32; 16],
    pub mode: u8,
    pub flags: u64,
}

impl Default for HilActuatorControls {
    fn default() -> Self {
        HilActuatorControls {
            header: crate::std_msgs::msg::Header::default(),
            controls: [0.0; 16],
            mode: 0,
            flags: 0,
        }
    }
}

impl ros2_client::Message for HilActuatorControls {}
