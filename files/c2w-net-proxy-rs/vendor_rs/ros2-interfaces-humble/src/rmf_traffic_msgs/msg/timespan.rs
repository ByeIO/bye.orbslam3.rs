use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Timespan {
    pub maps: Vec<::std::string::String>,
    pub has_lower_bound: bool,
    pub lower_bound: i64,
    pub has_upper_bound: bool,
    pub upper_bound: i64,
}

impl Default for Timespan {
    fn default() -> Self {
        Timespan {
            maps: Vec::new(),
            has_lower_bound: false,
            lower_bound: 0,
            has_upper_bound: false,
            upper_bound: 0,
        }
    }
}

impl ros2_client::Message for Timespan {}
