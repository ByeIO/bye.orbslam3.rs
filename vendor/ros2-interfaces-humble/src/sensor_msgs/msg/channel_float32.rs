use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChannelFloat32 {
    pub name: ::std::string::String,
    pub values: Vec<f32>,
}

impl Default for ChannelFloat32 {
    fn default() -> Self {
        ChannelFloat32 {
            name: ::std::string::String::new(),
            values: Vec::new(),
        }
    }
}

impl ros2_client::Message for ChannelFloat32 {}
