use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TurnSignalCmd {
    pub header: crate::std_msgs::msg::Header,
    pub cmd: crate::ds_dbw_msgs::msg::TurnSignal,
}

impl Default for TurnSignalCmd {
    fn default() -> Self {
        TurnSignalCmd {
            header: crate::std_msgs::msg::Header::default(),
            cmd: crate::ds_dbw_msgs::msg::TurnSignal::default(),
        }
    }
}

impl ros2_client::Message for TurnSignalCmd {}
