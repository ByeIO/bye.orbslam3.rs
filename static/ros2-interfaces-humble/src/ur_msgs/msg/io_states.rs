use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IOStates {
    pub digital_in_states: Vec<crate::ur_msgs::msg::Digital>,
    pub digital_out_states: Vec<crate::ur_msgs::msg::Digital>,
    pub flag_states: Vec<crate::ur_msgs::msg::Digital>,
    pub analog_in_states: Vec<crate::ur_msgs::msg::Analog>,
    pub analog_out_states: Vec<crate::ur_msgs::msg::Analog>,
}

impl Default for IOStates {
    fn default() -> Self {
        IOStates {
            digital_in_states: Vec::new(),
            digital_out_states: Vec::new(),
            flag_states: Vec::new(),
            analog_in_states: Vec::new(),
            analog_out_states: Vec::new(),
        }
    }
}

impl ros2_client::Message for IOStates {}
