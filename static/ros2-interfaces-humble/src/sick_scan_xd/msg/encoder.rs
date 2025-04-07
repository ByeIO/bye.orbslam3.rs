use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Encoder {
    pub header: crate::std_msgs::msg::Header,
    pub enc_position: i32,
    pub enc_speed: i16,
}

impl Default for Encoder {
    fn default() -> Self {
        Encoder {
            header: crate::std_msgs::msg::Header::default(),
            enc_position: 0,
            enc_speed: 0,
        }
    }
}

impl ros2_client::Message for Encoder {}
