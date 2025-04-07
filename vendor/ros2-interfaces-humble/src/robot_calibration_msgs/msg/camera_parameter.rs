use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraParameter {
    pub name: ::std::string::String,
    pub value: f64,
}

impl Default for CameraParameter {
    fn default() -> Self {
        CameraParameter {
            name: ::std::string::String::new(),
            value: 0.0,
        }
    }
}

impl ros2_client::Message for CameraParameter {}
