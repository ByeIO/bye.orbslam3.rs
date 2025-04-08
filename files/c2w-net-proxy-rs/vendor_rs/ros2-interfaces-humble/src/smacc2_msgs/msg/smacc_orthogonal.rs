use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SmaccOrthogonal {
    pub name: ::std::string::String,
    pub client_behavior_names: Vec<::std::string::String>,
    pub client_names: Vec<::std::string::String>,
}

impl Default for SmaccOrthogonal {
    fn default() -> Self {
        SmaccOrthogonal {
            name: ::std::string::String::new(),
            client_behavior_names: Vec::new(),
            client_names: Vec::new(),
        }
    }
}

impl ros2_client::Message for SmaccOrthogonal {}
