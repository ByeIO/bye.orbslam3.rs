use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LookAtWithStyle {
    pub style: i8,
    pub target: crate::geometry_msgs::msg::PointStamped,
}

impl LookAtWithStyle {
    pub const DEFAULT: i8 = 0;
    pub const EYES_ONLY: i8 = 1;
    pub const HEAD_ONLY: i8 = 2;
}

impl Default for LookAtWithStyle {
    fn default() -> Self {
        LookAtWithStyle {
            style: 0,
            target: crate::geometry_msgs::msg::PointStamped::default(),
        }
    }
}

impl ros2_client::Message for LookAtWithStyle {}
