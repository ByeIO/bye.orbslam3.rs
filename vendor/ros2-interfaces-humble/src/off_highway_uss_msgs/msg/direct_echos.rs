use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DirectEchos {
    pub header: crate::std_msgs::msg::Header,
    pub direct_echos: Vec<crate::off_highway_uss_msgs::msg::DirectEcho>,
}

impl Default for DirectEchos {
    fn default() -> Self {
        DirectEchos {
            header: crate::std_msgs::msg::Header::default(),
            direct_echos: Vec::new(),
        }
    }
}

impl ros2_client::Message for DirectEchos {}
