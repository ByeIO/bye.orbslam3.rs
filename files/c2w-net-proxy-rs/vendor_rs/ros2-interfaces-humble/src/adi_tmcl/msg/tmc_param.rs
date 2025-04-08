use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TmcParam {
    pub name: ::std::string::String,
    pub value: i64,
}

impl Default for TmcParam {
    fn default() -> Self {
        TmcParam {
            name: ::std::string::String::new(),
            value: 0,
        }
    }
}

impl ros2_client::Message for TmcParam {}
