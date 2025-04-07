use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LinkPadding {
    pub link_name: ::std::string::String,
    pub padding: f64,
}

impl Default for LinkPadding {
    fn default() -> Self {
        LinkPadding {
            link_name: ::std::string::String::new(),
            padding: 0.0,
        }
    }
}

impl ros2_client::Message for LinkPadding {}
