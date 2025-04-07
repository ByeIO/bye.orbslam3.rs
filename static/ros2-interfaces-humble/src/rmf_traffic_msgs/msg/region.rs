use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Region {
    pub map: ::std::string::String,
    pub spaces: Vec<crate::rmf_traffic_msgs::msg::Space>,
    pub timespan: crate::rmf_traffic_msgs::msg::Timespan,
}

impl Default for Region {
    fn default() -> Self {
        Region {
            map: ::std::string::String::new(),
            spaces: Vec::new(),
            timespan: crate::rmf_traffic_msgs::msg::Timespan::default(),
        }
    }
}

impl ros2_client::Message for Region {}
