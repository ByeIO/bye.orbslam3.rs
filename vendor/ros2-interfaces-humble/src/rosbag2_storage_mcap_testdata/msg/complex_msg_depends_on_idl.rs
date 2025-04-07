use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComplexMsgDependsOnIdl {
    pub a: crate::rosbag2_storage_mcap_testdata::msg::BasicIdl,
}

impl Default for ComplexMsgDependsOnIdl {
    fn default() -> Self {
        ComplexMsgDependsOnIdl {
            a: crate::rosbag2_storage_mcap_testdata::msg::BasicIdl::default(),
        }
    }
}

impl ros2_client::Message for ComplexMsgDependsOnIdl {}
