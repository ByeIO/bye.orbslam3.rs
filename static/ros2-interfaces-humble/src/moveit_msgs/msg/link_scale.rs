use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LinkScale {
    pub link_name: ::std::string::String,
    pub scale: f64,
}

impl Default for LinkScale {
    fn default() -> Self {
        LinkScale {
            link_name: ::std::string::String::new(),
            scale: 0.0,
        }
    }
}

impl ros2_client::Message for LinkScale {}
