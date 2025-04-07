use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Float32MultiArray {
    pub layout: crate::std_msgs::msg::MultiArrayLayout,
    pub data: Vec<f32>,
}

impl Default for Float32MultiArray {
    fn default() -> Self {
        Float32MultiArray {
            layout: crate::std_msgs::msg::MultiArrayLayout::default(),
            data: Vec::new(),
        }
    }
}

impl ros2_client::Message for Float32MultiArray {}
