use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EsrTrackMotionPowerGroup {
    pub header: crate::std_msgs::msg::Header,
    pub canmsg: ::std::string::String,
    pub rolling_count_2: u8,
    pub can_id_group: u8,
    pub tracks: Vec<crate::delphi_esr_msgs::msg::EsrTrackMotionPowerTrack>,
}

impl Default for EsrTrackMotionPowerGroup {
    fn default() -> Self {
        EsrTrackMotionPowerGroup {
            header: crate::std_msgs::msg::Header::default(),
            canmsg: ::std::string::String::new(),
            rolling_count_2: 0,
            can_id_group: 0,
            tracks: Vec::new(),
        }
    }
}

impl ros2_client::Message for EsrTrackMotionPowerGroup {}
