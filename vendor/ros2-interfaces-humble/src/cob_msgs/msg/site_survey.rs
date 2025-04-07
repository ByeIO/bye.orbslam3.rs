use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SiteSurvey {
    pub header: crate::std_msgs::msg::Header,
    pub networks: Vec<crate::cob_msgs::msg::Network>,
}

impl Default for SiteSurvey {
    fn default() -> Self {
        SiteSurvey {
            header: crate::std_msgs::msg::Header::default(),
            networks: Vec::new(),
        }
    }
}

impl ros2_client::Message for SiteSurvey {}
