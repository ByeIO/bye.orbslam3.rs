use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MutexGroupManualRelease {
    pub release_mutex_groups: Vec<::std::string::String>,
    pub fleet: ::std::string::String,
    pub robot: ::std::string::String,
}

impl Default for MutexGroupManualRelease {
    fn default() -> Self {
        MutexGroupManualRelease {
            release_mutex_groups: Vec::new(),
            fleet: ::std::string::String::new(),
            robot: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for MutexGroupManualRelease {}
