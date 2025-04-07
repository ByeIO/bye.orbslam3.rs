use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Packet {
    pub header: crate::std_msgs::msg::Header,
    pub data: Vec<u8>,
    pub b_o_s: i32,
    pub e_o_s: i32,
    pub granulepos: i64,
    pub packetno: i64,
}

impl Default for Packet {
    fn default() -> Self {
        Packet {
            header: crate::std_msgs::msg::Header::default(),
            data: Vec::new(),
            b_o_s: 0,
            e_o_s: 0,
            granulepos: 0,
            packetno: 0,
        }
    }
}

impl ros2_client::Message for Packet {}
