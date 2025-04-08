use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Statistics {
    pub header: crate::std_msgs::msg::Header,
    pub statistics: Vec<crate::pal_statistics_msgs::msg::Statistic>,
}

impl Default for Statistics {
    fn default() -> Self {
        Statistics {
            header: crate::std_msgs::msg::Header::default(),
            statistics: Vec::new(),
        }
    }
}

impl ros2_client::Message for Statistics {}
