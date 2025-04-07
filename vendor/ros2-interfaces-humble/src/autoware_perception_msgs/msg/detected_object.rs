use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DetectedObject {
    pub existence_probability: f32,
    pub classification: Vec<crate::autoware_perception_msgs::msg::ObjectClassification>,
    pub kinematics: crate::autoware_perception_msgs::msg::DetectedObjectKinematics,
    pub shape: crate::autoware_perception_msgs::msg::Shape,
}

impl Default for DetectedObject {
    fn default() -> Self {
        DetectedObject {
            existence_probability: 0.0,
            classification: Vec::new(),
            kinematics: crate::autoware_perception_msgs::msg::DetectedObjectKinematics::default(),
            shape: crate::autoware_perception_msgs::msg::Shape::default(),
        }
    }
}

impl ros2_client::Message for DetectedObject {}
