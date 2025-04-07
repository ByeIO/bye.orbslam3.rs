use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MrrHeaderAlignmentState {
    pub header: crate::std_msgs::msg::Header,
    pub can_auto_align_hangle_qf: u8,
    pub can_alignment_status: u8,
    pub can_alignment_state: u8,
    pub can_auto_align_hangle_ref: f32,
    pub can_auto_align_hangle: f32,
}

impl Default for MrrHeaderAlignmentState {
    fn default() -> Self {
        MrrHeaderAlignmentState {
            header: crate::std_msgs::msg::Header::default(),
            can_auto_align_hangle_qf: 0,
            can_alignment_status: 0,
            can_alignment_state: 0,
            can_auto_align_hangle_ref: 0.0,
            can_auto_align_hangle: 0.0,
        }
    }
}

impl ros2_client::Message for MrrHeaderAlignmentState {}
