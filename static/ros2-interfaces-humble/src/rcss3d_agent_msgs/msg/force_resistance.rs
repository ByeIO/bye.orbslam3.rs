use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForceResistance {
    pub name: ::std::string::String,
    pub px: f32,
    pub py: f32,
    pub pz: f32,
    pub fx: f32,
    pub fy: f32,
    pub fz: f32,
}

impl Default for ForceResistance {
    fn default() -> Self {
        ForceResistance {
            name: ::std::string::String::new(),
            px: 0.0,
            py: 0.0,
            pz: 0.0,
            fx: 0.0,
            fy: 0.0,
            fz: 0.0,
        }
    }
}

impl ros2_client::Message for ForceResistance {}
