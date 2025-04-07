use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SynthesisRequest {
    pub name: ::std::string::String,
    pub system: ::std::string::String,
    pub goal: ::std::string::String,
    pub initial_condition: ::std::string::String,
    pub sm_outcomes: Vec<::std::string::String>,
}

impl Default for SynthesisRequest {
    fn default() -> Self {
        SynthesisRequest {
            name: ::std::string::String::new(),
            system: ::std::string::String::new(),
            goal: ::std::string::String::new(),
            initial_condition: ::std::string::String::new(),
            sm_outcomes: Vec::new(),
        }
    }
}

impl ros2_client::Message for SynthesisRequest {}
