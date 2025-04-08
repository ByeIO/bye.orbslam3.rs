use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NovatelSignalMask {
    pub original_mask: u32,
    pub gps_l1_used_in_solution: bool,
    pub gps_l2_used_in_solution: bool,
    pub gps_l3_used_in_solution: bool,
    pub glonass_l1_used_in_solution: bool,
    pub glonass_l2_used_in_solution: bool,
}

impl Default for NovatelSignalMask {
    fn default() -> Self {
        NovatelSignalMask {
            original_mask: 0,
            gps_l1_used_in_solution: false,
            gps_l2_used_in_solution: false,
            gps_l3_used_in_solution: false,
            glonass_l1_used_in_solution: false,
            glonass_l2_used_in_solution: false,
        }
    }
}

impl ros2_client::Message for NovatelSignalMask {}
