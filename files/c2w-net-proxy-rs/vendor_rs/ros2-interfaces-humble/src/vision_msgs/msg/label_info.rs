use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LabelInfo {
    pub header: crate::std_msgs::msg::Header,
    pub class_map: Vec<crate::vision_msgs::msg::VisionClass>,
    pub threshold: f32,
}

impl Default for LabelInfo {
    fn default() -> Self {
        LabelInfo {
            header: crate::std_msgs::msg::Header::default(),
            class_map: Vec::new(),
            threshold: 0.0,
        }
    }
}

impl ros2_client::Message for LabelInfo {}
