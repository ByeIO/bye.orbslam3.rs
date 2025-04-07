use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatisticsValues {
    pub header: crate::std_msgs::msg::Header,
    pub values: Vec<f64>,
    pub names_version: u32,
}

impl Default for StatisticsValues {
    fn default() -> Self {
        StatisticsValues {
            header: crate::std_msgs::msg::Header::default(),
            values: Vec::new(),
            names_version: 0,
        }
    }
}

impl ros2_client::Message for StatisticsValues {}
