use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Weight {
    pub stamp: crate::builtin_interfaces::msg::Time,
    pub stable: bool,
    pub overload: bool,
    pub weight: f64,
    pub unit: ::std::string::String,
}

impl Default for Weight {
    fn default() -> Self {
        Weight {
            stamp: crate::builtin_interfaces::msg::Time::default(),
            stable: false,
            overload: false,
            weight: 0.0,
            unit: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for Weight {}
