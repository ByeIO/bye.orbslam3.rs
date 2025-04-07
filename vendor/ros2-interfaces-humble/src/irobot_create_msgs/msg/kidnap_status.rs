use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KidnapStatus {
    pub header: crate::std_msgs::msg::Header,
    pub is_kidnapped: bool,
}

impl Default for KidnapStatus {
    fn default() -> Self {
        KidnapStatus {
            header: crate::std_msgs::msg::Header::default(),
            is_kidnapped: false,
        }
    }
}

impl ros2_client::Message for KidnapStatus {}
