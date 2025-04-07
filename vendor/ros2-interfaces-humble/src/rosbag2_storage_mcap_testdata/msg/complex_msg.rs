use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComplexMsg {
    pub b: crate::rosbag2_storage_mcap_testdata::msg::BasicMsg,
}

impl Default for ComplexMsg {
    fn default() -> Self {
        ComplexMsg {
            b: crate::rosbag2_storage_mcap_testdata::msg::BasicMsg::default(),
        }
    }
}

impl ros2_client::Message for ComplexMsg {}
