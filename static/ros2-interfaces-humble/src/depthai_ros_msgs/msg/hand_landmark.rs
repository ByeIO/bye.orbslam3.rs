use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HandLandmark {
    pub label: ::std::string::String,
    pub lm_score: f32,
    pub landmark: Vec<crate::geometry_msgs::msg::Pose2D>,
    pub position: crate::geometry_msgs::msg::Point,
    pub is_spatial: bool,
}

impl Default for HandLandmark {
    fn default() -> Self {
        HandLandmark {
            label: ::std::string::String::new(),
            lm_score: 0.0,
            landmark: Vec::new(),
            position: crate::geometry_msgs::msg::Point::default(),
            is_spatial: false,
        }
    }
}

impl ros2_client::Message for HandLandmark {}
