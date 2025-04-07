use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VersionInfo {
    pub hardware: ::std::string::String,
    pub firmware: ::std::string::String,
    pub software: ::std::string::String,
    pub udid: Vec<u32>,
    pub features: u64,
}

impl VersionInfo {
    pub const SMOOTH_MOVE_START: u64 = 0000000000000001;
    pub const GYROSCOPE_3D_DATA: u64 = 0000000000000002;
}

impl Default for VersionInfo {
    fn default() -> Self {
        VersionInfo {
            hardware: ::std::string::String::new(),
            firmware: ::std::string::String::new(),
            software: ::std::string::String::new(),
            udid: Vec::new(),
            features: 0,
        }
    }
}

impl ros2_client::Message for VersionInfo {}
