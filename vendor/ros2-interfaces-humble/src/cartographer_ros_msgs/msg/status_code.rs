use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatusCode {

}

impl StatusCode {
    pub const OK: u8 = 0;
    pub const CANCELLED: u8 = 1;
    pub const UNKNOWN: u8 = 2;
    pub const INVALID_ARGUMENT: u8 = 3;
    pub const DEADLINE_EXCEEDED: u8 = 4;
    pub const NOT_FOUND: u8 = 5;
    pub const ALREADY_EXISTS: u8 = 6;
    pub const PERMISSION_DENIED: u8 = 7;
    pub const RESOURCE_EXHAUSTED: u8 = 8;
    pub const FAILED_PRECONDITION: u8 = 9;
    pub const ABORTED: u8 = 10;
    pub const OUT_OF_RANGE: u8 = 11;
    pub const UNIMPLEMENTED: u8 = 12;
    pub const INTERNAL: u8 = 13;
    pub const UNAVAILABLE: u8 = 14;
    pub const DATA_LOSS: u8 = 15;
}

impl Default for StatusCode {
    fn default() -> Self {
        StatusCode {

        }
    }
}

impl ros2_client::Message for StatusCode {}
