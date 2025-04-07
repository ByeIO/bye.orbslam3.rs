use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Float64MultiArray {
    pub layout: crate::std_msgs::msg::MultiArrayLayout,
    pub data: Vec<f64>,
}

impl Default for Float64MultiArray {
    fn default() -> Self {
        Float64MultiArray {
            layout: crate::std_msgs::msg::MultiArrayLayout::default(),
            data: Vec::new(),
        }
    }
}

impl ros2_client::Message for Float64MultiArray {}
