use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SrrStatus4 {
    pub header: crate::std_msgs::msg::Header,
    pub can_tx_sw_version_host: u16,
    pub can_tx_path_id_blis_ignore: u8,
    pub can_tx_path_id_blis: u8,
    pub can_tx_angle_misalignment: f32,
    pub can_tx_auto_align_angle: f32,
}

impl Default for SrrStatus4 {
    fn default() -> Self {
        SrrStatus4 {
            header: crate::std_msgs::msg::Header::default(),
            can_tx_sw_version_host: 0,
            can_tx_path_id_blis_ignore: 0,
            can_tx_path_id_blis: 0,
            can_tx_angle_misalignment: 0.0,
            can_tx_auto_align_angle: 0.0,
        }
    }
}

impl ros2_client::Message for SrrStatus4 {}
