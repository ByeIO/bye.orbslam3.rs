use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SegmentArray {
    pub header: crate::std_msgs::msg::Header,
    pub segments: Vec<crate::slg_msgs::msg::Segment>,
}

impl Default for SegmentArray {
    fn default() -> Self {
        SegmentArray {
            header: crate::std_msgs::msg::Header::default(),
            segments: Vec::new(),
        }
    }
}

impl ros2_client::Message for SegmentArray {}
