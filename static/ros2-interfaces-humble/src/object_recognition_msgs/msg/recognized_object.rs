use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecognizedObject {
    pub header: crate::std_msgs::msg::Header,
    #[serde(rename = "type")]    pub type_: crate::object_recognition_msgs::msg::ObjectType,
    pub confidence: f32,
    pub point_clouds: Vec<crate::sensor_msgs::msg::PointCloud2>,
    pub bounding_mesh: crate::shape_msgs::msg::Mesh,
    pub bounding_contours: Vec<crate::geometry_msgs::msg::Point>,
    pub pose: crate::geometry_msgs::msg::PoseWithCovarianceStamped,
}

impl Default for RecognizedObject {
    fn default() -> Self {
        RecognizedObject {
            header: crate::std_msgs::msg::Header::default(),
            type_: crate::object_recognition_msgs::msg::ObjectType::default(),
            confidence: 0.0,
            point_clouds: Vec::new(),
            bounding_mesh: crate::shape_msgs::msg::Mesh::default(),
            bounding_contours: Vec::new(),
            pose: crate::geometry_msgs::msg::PoseWithCovarianceStamped::default(),
        }
    }
}

impl ros2_client::Message for RecognizedObject {}
