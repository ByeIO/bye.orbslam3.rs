use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IdsList {
    pub header: crate::std_msgs::msg::Header,
    pub ids: Vec<::std::string::String>,
}

impl Default for IdsList {
    fn default() -> Self {
        IdsList {
            header: crate::std_msgs::msg::Header::default(),
            ids: Vec::new(),
        }
    }
}

impl ros2_client::Message for IdsList {}
