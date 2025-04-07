use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hear {
    pub team: ::std::string::String,
    pub time: f32,
    #[serde(rename = "self")]    pub self_: bool,
    pub direction: Vec<f32>,
    pub message: ::std::string::String,
}

impl Default for Hear {
    fn default() -> Self {
        Hear {
            team: ::std::string::String::new(),
            time: 0.0,
            self_: false,
            direction: Vec::new(),
            message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for Hear {}
