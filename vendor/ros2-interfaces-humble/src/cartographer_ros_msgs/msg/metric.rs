use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Metric {
    #[serde(rename = "type")]    pub type_: u8,
    pub labels: Vec<crate::cartographer_ros_msgs::msg::MetricLabel>,
    pub value: f64,
    pub counts_by_bucket: Vec<crate::cartographer_ros_msgs::msg::HistogramBucket>,
}

impl Metric {
    pub const TYPE_COUNTER: u8 = 0;
    pub const TYPE_GAUGE: u8 = 1;
    pub const TYPE_HISTOGRAM: u8 = 2;
}

impl Default for Metric {
    fn default() -> Self {
        Metric {
            type_: 0,
            labels: Vec::new(),
            value: 0.0,
            counts_by_bucket: Vec::new(),
        }
    }
}

impl ros2_client::Message for Metric {}
