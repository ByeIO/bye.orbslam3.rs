use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BallArray {
    pub header: crate::std_msgs::msg::Header,
    pub balls: Vec<crate::soccer_vision_3d_msgs::msg::Ball>,
}

impl Default for BallArray {
    fn default() -> Self {
        BallArray {
            header: crate::std_msgs::msg::Header::default(),
            balls: Vec::new(),
        }
    }
}

impl ros2_client::Message for BallArray {}
