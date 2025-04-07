use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttEuler {
    pub header: crate::std_msgs::msg::Header,
    pub block_header: crate::septentrio_gnss_driver::msg::BlockHeader,
    pub nr_sv: u8,
    pub error: u8,
    pub mode: u16,
    pub heading: f32,
    pub pitch: f32,
    pub roll: f32,
    pub pitch_dot: f32,
    pub roll_dot: f32,
    pub heading_dot: f32,
}

impl Default for AttEuler {
    fn default() -> Self {
        AttEuler {
            header: crate::std_msgs::msg::Header::default(),
            block_header: crate::septentrio_gnss_driver::msg::BlockHeader::default(),
            nr_sv: 0,
            error: 0,
            mode: 0,
            heading: 0.0,
            pitch: 0.0,
            roll: 0.0,
            pitch_dot: 0.0,
            roll_dot: 0.0,
            heading_dot: 0.0,
        }
    }
}

impl ros2_client::Message for AttEuler {}
