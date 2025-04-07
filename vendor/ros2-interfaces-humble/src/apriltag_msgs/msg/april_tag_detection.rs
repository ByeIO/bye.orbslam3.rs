use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AprilTagDetection {
    pub family: ::std::string::String,
    pub id: i32,
    pub hamming: i32,
    pub goodness: f32,
    pub decision_margin: f32,
    pub centre: crate::apriltag_msgs::msg::Point,
    pub corners: [crate::apriltag_msgs::msg::Point; 4],
    pub homography: [f64; 9],
}

impl Default for AprilTagDetection {
    fn default() -> Self {
        AprilTagDetection {
            family: ::std::string::String::new(),
            id: 0,
            hamming: 0,
            goodness: 0.0,
            decision_margin: 0.0,
            centre: crate::apriltag_msgs::msg::Point::default(),
            corners: core::array::from_fn(|_| crate::apriltag_msgs::msg::Point::default()),
            homography: [0.0; 9],
        }
    }
}

impl ros2_client::Message for AprilTagDetection {}
