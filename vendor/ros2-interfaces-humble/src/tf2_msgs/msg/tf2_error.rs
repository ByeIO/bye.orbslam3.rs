use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TF2Error {
    pub error: u8,
    pub error_string: ::std::string::String,
}

impl TF2Error {
    pub const NO_ERROR: u8 = 0;
    pub const LOOKUP_ERROR: u8 = 1;
    pub const CONNECTIVITY_ERROR: u8 = 2;
    pub const EXTRAPOLATION_ERROR: u8 = 3;
    pub const INVALID_ARGUMENT_ERROR: u8 = 4;
    pub const TIMEOUT_ERROR: u8 = 5;
    pub const TRANSFORM_ERROR: u8 = 6;
}

impl Default for TF2Error {
    fn default() -> Self {
        TF2Error {
            error: 0,
            error_string: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for TF2Error {}
