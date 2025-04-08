use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrackedObject {
    pub object_id: crate::unique_identifier_msgs::msg::UUID,
    pub existence_probability: f32,
    pub classification: Vec<crate::autoware_perception_msgs::msg::ObjectClassification>,
    pub kinematics: crate::autoware_perception_msgs::msg::TrackedObjectKinematics,
    pub shape: crate::autoware_perception_msgs::msg::Shape,
}

impl Default for TrackedObject {
    fn default() -> Self {
        TrackedObject {
            object_id: crate::unique_identifier_msgs::msg::UUID::default(),
            existence_probability: 0.0,
            classification: Vec::new(),
            kinematics: crate::autoware_perception_msgs::msg::TrackedObjectKinematics::default(),
            shape: crate::autoware_perception_msgs::msg::Shape::default(),
        }
    }
}

impl ros2_client::Message for TrackedObject {}
