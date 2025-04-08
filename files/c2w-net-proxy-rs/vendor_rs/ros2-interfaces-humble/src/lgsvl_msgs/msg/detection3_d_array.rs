use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Detection3DArray {
    pub header: crate::std_msgs::msg::Header,
    pub detections: Vec<crate::lgsvl_msgs::msg::Detection3D>,
}

impl Default for Detection3DArray {
    fn default() -> Self {
        Detection3DArray {
            header: crate::std_msgs::msg::Header::default(),
            detections: Vec::new(),
        }
    }
}

impl ros2_client::Message for Detection3DArray {}
