use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Buttons {
    pub chest: bool,
    pub l_foot_bumper_left: bool,
    pub l_foot_bumper_right: bool,
    pub r_foot_bumper_left: bool,
    pub r_foot_bumper_right: bool,
}

impl Default for Buttons {
    fn default() -> Self {
        Buttons {
            chest: false,
            l_foot_bumper_left: false,
            l_foot_bumper_right: false,
            r_foot_bumper_left: false,
            r_foot_bumper_right: false,
        }
    }
}

impl ros2_client::Message for Buttons {}
