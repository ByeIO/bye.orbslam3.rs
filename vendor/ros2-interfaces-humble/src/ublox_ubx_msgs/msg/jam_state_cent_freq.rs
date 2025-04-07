use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JamStateCentFreq {
    pub cent_freq: u32,
    pub jammed: bool,
}

impl Default for JamStateCentFreq {
    fn default() -> Self {
        JamStateCentFreq {
            cent_freq: 0,
            jammed: false,
        }
    }
}

impl ros2_client::Message for JamStateCentFreq {}
