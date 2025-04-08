use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Node {
    pub id: i32,
    pub map_id: i32,
    pub weight: i32,
    pub stamp: f64,
    pub label: ::std::string::String,
    pub pose: crate::geometry_msgs::msg::Pose,
    pub word_id_keys: Vec<i32>,
    pub word_id_values: Vec<i32>,
    pub word_kpts: Vec<crate::rtabmap_msgs::msg::KeyPoint>,
    pub word_pts: Vec<crate::rtabmap_msgs::msg::Point3f>,
    pub word_descriptors: Vec<u8>,
    pub data: crate::rtabmap_msgs::msg::SensorData,
}

impl Default for Node {
    fn default() -> Self {
        Node {
            id: 0,
            map_id: 0,
            weight: 0,
            stamp: 0.0,
            label: ::std::string::String::new(),
            pose: crate::geometry_msgs::msg::Pose::default(),
            word_id_keys: Vec::new(),
            word_id_values: Vec::new(),
            word_kpts: Vec::new(),
            word_pts: Vec::new(),
            word_descriptors: Vec::new(),
            data: crate::rtabmap_msgs::msg::SensorData::default(),
        }
    }
}

impl ros2_client::Message for Node {}
