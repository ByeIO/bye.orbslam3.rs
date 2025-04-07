use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StateInstantiation {
    pub state_path: ::std::string::String,
    pub state_class: ::std::string::String,
    pub initial_state_name: ::std::string::String,
    pub input_keys: Vec<::std::string::String>,
    pub output_keys: Vec<::std::string::String>,
    pub cond_outcome: Vec<::std::string::String>,
    pub cond_transition: Vec<crate::flexbe_msgs::msg::OutcomeCondition>,
    pub behavior_class: ::std::string::String,
    pub parameter_names: Vec<::std::string::String>,
    pub parameter_values: Vec<::std::string::String>,
    pub position: [f32; 2],
    pub outcomes: Vec<::std::string::String>,
    pub transitions: Vec<::std::string::String>,
    pub autonomy: Vec<i8>,
    pub userdata_keys: Vec<::std::string::String>,
    pub userdata_remapping: Vec<::std::string::String>,
}

impl StateInstantiation {
    pub const CLASS_STATEMACHINE: &'static str = ":STATEMACHINE";
    pub const CLASS_CONCURRENCY: &'static str = ":CONCURRENCY";
    pub const CLASS_PRIORITY: &'static str = ":PRIORITY";
    pub const CLASS_BEHAVIOR: &'static str = ":BEHAVIOR";
}

impl Default for StateInstantiation {
    fn default() -> Self {
        StateInstantiation {
            state_path: ::std::string::String::new(),
            state_class: ::std::string::String::new(),
            initial_state_name: ::std::string::String::new(),
            input_keys: Vec::new(),
            output_keys: Vec::new(),
            cond_outcome: Vec::new(),
            cond_transition: Vec::new(),
            behavior_class: ::std::string::String::new(),
            parameter_names: Vec::new(),
            parameter_values: Vec::new(),
            position: [0.0; 2],
            outcomes: Vec::new(),
            transitions: Vec::new(),
            autonomy: Vec::new(),
            userdata_keys: Vec::new(),
            userdata_remapping: Vec::new(),
        }
    }
}

impl ros2_client::Message for StateInstantiation {}
