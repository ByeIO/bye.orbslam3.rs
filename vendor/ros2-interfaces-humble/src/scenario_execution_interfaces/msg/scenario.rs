use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Scenario {
    pub name: ::std::string::String,
    pub scenario_file: ::std::string::String,
}

impl Default for Scenario {
    fn default() -> Self {
        Scenario {
            name: ::std::string::String::new(),
            scenario_file: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for Scenario {}
