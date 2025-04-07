use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ahbc {
    pub header: crate::std_msgs::msg::Header,
    pub high_low_beam_decision: u8,
    pub reasons_for_switch_to_low_beam: u16,
}

impl Ahbc {
    pub const HIGH_LOW_BEAM_DECISION_NO_RECOMMENDATION: u8 = 0;
    pub const HIGH_LOW_BEAM_DECISION_RECOMMENDATION_OFF: u8 = 1;
    pub const HIGH_LOW_BEAM_DECISION_RECOMMENDATION_ON: u8 = 2;
    pub const HIGH_LOW_BEAM_DECISION_INVALID: u8 = 3;
}

impl Default for Ahbc {
    fn default() -> Self {
        Ahbc {
            header: crate::std_msgs::msg::Header::default(),
            high_low_beam_decision: 0,
            reasons_for_switch_to_low_beam: 0,
        }
    }
}

impl ros2_client::Message for Ahbc {}
