use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Switches {
    pub switch0: bool,
    pub switch1: bool,
    pub switch2: bool,
}

impl Default for Switches {
    fn default() -> Self {
        Switches {
            switch0: false,
            switch1: false,
            switch2: false,
        }
    }
}

impl ros2_client::Message for Switches {}
