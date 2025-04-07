use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BeaconState {
    pub id: ::std::string::String,
    pub online: bool,
    pub category: ::std::string::String,
    pub activated: bool,
    pub level: ::std::string::String,
}

impl Default for BeaconState {
    fn default() -> Self {
        BeaconState {
            id: ::std::string::String::new(),
            online: false,
            category: ::std::string::String::new(),
            activated: false,
            level: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for BeaconState {}
