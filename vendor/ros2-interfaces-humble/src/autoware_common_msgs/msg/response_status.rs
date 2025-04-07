use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResponseStatus {
    pub success: bool,
    pub code: u16,
    pub message: ::std::string::String,
}

impl ResponseStatus {
    pub const UNKNOWN: u16 = 50000;
    pub const SERVICE_UNREADY: u16 = 50001;
    pub const SERVICE_TIMEOUT: u16 = 50002;
    pub const TRANSFORM_ERROR: u16 = 50003;
    pub const PARAMETER_ERROR: u16 = 50004;
    pub const DEPRECATED: u16 = 60000;
    pub const NO_EFFECT: u16 = 60001;
}

impl Default for ResponseStatus {
    fn default() -> Self {
        ResponseStatus {
            success: false,
            code: 0,
            message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ResponseStatus {}
