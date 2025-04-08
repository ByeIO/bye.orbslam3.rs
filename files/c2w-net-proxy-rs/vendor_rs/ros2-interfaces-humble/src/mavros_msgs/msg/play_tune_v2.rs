use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayTuneV2 {
    pub format: u8,
    pub tune: ::std::string::String,
}

impl PlayTuneV2 {
    pub const QBASIC1_1: u8 = 1;
    pub const MML_MODERN: u8 = 2;
}

impl Default for PlayTuneV2 {
    fn default() -> Self {
        PlayTuneV2 {
            format: 0,
            tune: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for PlayTuneV2 {}
