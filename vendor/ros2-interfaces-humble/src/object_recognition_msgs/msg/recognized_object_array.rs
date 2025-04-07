use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecognizedObjectArray {
    pub header: crate::std_msgs::msg::Header,
    pub objects: Vec<crate::object_recognition_msgs::msg::RecognizedObject>,
    pub cooccurrence: Vec<f32>,
}

impl Default for RecognizedObjectArray {
    fn default() -> Self {
        RecognizedObjectArray {
            header: crate::std_msgs::msg::Header::default(),
            objects: Vec::new(),
            cooccurrence: Vec::new(),
        }
    }
}

impl ros2_client::Message for RecognizedObjectArray {}
