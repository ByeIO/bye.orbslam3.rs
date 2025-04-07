use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NovatelPsrdop2System {
    pub system: ::std::string::String,
    pub tdop: f32,
}

impl Default for NovatelPsrdop2System {
    fn default() -> Self {
        NovatelPsrdop2System {
            system: ::std::string::String::new(),
            tdop: 0.0,
        }
    }
}

impl ros2_client::Message for NovatelPsrdop2System {}
