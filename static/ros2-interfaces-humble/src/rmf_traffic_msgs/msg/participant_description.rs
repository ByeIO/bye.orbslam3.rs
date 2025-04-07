use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParticipantDescription {
    pub name: ::std::string::String,
    pub owner: ::std::string::String,
    pub responsiveness: u8,
    pub profile: crate::rmf_traffic_msgs::msg::Profile,
}

impl ParticipantDescription {
    pub const RX_INVALID: u8 = 0;
    pub const RX_UNRESPONSIVE: u8 = 1;
    pub const RX_RESPONSIVE: u8 = 2;
}

impl Default for ParticipantDescription {
    fn default() -> Self {
        ParticipantDescription {
            name: ::std::string::String::new(),
            owner: ::std::string::String::new(),
            responsiveness: 0,
            profile: crate::rmf_traffic_msgs::msg::Profile::default(),
        }
    }
}

impl ros2_client::Message for ParticipantDescription {}
