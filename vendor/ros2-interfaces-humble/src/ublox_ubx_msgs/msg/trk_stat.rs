use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrkStat {
    pub pr_valid: bool,
    pub cp_valid: bool,
    pub half_cyc: bool,
    pub sub_half_cyc: bool,
}

impl Default for TrkStat {
    fn default() -> Self {
        TrkStat {
            pr_valid: false,
            cp_valid: false,
            half_cyc: false,
            sub_half_cyc: false,
        }
    }
}

impl ros2_client::Message for TrkStat {}
