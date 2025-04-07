use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WheelPositions {
    pub header: crate::std_msgs::msg::Header,
    pub front_left: i16,
    pub front_right: i16,
    pub rear_left: i16,
    pub rear_right: i16,
}

impl Default for WheelPositions {
    fn default() -> Self {
        WheelPositions {
            header: crate::std_msgs::msg::Header::default(),
            front_left: 0,
            front_right: 0,
            rear_left: 0,
            rear_right: 0,
        }
    }
}

impl ros2_client::Message for WheelPositions {}
