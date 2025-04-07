use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Feedback {
    pub header: crate::std_msgs::msg::Header,
    pub drivers: [crate::clearpath_platform_msgs::msg::DriveFeedback; 2],
    pub commanded_mode: i8,
    pub actual_mode: i8,
}

impl Default for Feedback {
    fn default() -> Self {
        Feedback {
            header: crate::std_msgs::msg::Header::default(),
            drivers: core::array::from_fn(|_| crate::clearpath_platform_msgs::msg::DriveFeedback::default()),
            commanded_mode: 0,
            actual_mode: 0,
        }
    }
}

impl ros2_client::Message for Feedback {}
