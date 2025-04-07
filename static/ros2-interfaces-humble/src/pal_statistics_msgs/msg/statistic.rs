use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Statistic {
    pub name: ::std::string::String,
    pub value: f64,
}

impl Default for Statistic {
    fn default() -> Self {
        Statistic {
            name: ::std::string::String::new(),
            value: 0.0,
        }
    }
}

impl ros2_client::Message for Statistic {}
