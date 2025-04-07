use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Info {
    pub id: i32,
    pub serial_port: ::std::string::String,
    pub max_repeats: i32,
    pub get_positions: bool,
    pub get_currents: bool,
    pub get_distinct_packages: bool,
    pub set_commands: bool,
    pub set_commands_async: bool,
    pub position_limits: Vec<i32>,
    pub encoder_resolutions: Vec<u8>,
}

impl Default for Info {
    fn default() -> Self {
        Info {
            id: 0,
            serial_port: ::std::string::String::new(),
            max_repeats: 0,
            get_positions: false,
            get_currents: false,
            get_distinct_packages: false,
            set_commands: false,
            set_commands_async: false,
            position_limits: Vec::new(),
            encoder_resolutions: Vec::new(),
        }
    }
}

impl ros2_client::Message for Info {}
