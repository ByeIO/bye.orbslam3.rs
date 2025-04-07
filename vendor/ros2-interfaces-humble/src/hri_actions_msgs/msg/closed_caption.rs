use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClosedCaption {
    pub speaker_id: ::std::string::String,
    pub text: ::std::string::String,
    pub locale: ::std::string::String,
}

impl ClosedCaption {
    pub const SPEAKER_ID_SYSTEM: &'static str = "__system__";
    pub const SPEAKER_ID_UNKNOWN: &'static str = "__unknown__";
}

impl Default for ClosedCaption {
    fn default() -> Self {
        ClosedCaption {
            speaker_id: ::std::string::String::new(),
            text: ::std::string::String::new(),
            locale: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ClosedCaption {}
