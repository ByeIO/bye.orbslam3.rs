use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MrrHeaderInformationDetections {
    pub header: crate::std_msgs::msg::Header,
    pub can_align_updates_done: u16,
    pub can_scan_index: u16,
    pub can_number_of_det: u8,
    pub can_look_id: u8,
    pub can_look_index: u16,
}

impl Default for MrrHeaderInformationDetections {
    fn default() -> Self {
        MrrHeaderInformationDetections {
            header: crate::std_msgs::msg::Header::default(),
            can_align_updates_done: 0,
            can_scan_index: 0,
            can_number_of_det: 0,
            can_look_id: 0,
            can_look_index: 0,
        }
    }
}

impl ros2_client::Message for MrrHeaderInformationDetections {}
