use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MiscCmd {
    pub header: crate::std_msgs::msg::Header,
    pub parking_brake: crate::ds_dbw_msgs::msg::PrkBrkCmd,
}

impl Default for MiscCmd {
    fn default() -> Self {
        MiscCmd {
            header: crate::std_msgs::msg::Header::default(),
            parking_brake: crate::ds_dbw_msgs::msg::PrkBrkCmd::default(),
        }
    }
}

impl ros2_client::Message for MiscCmd {}
