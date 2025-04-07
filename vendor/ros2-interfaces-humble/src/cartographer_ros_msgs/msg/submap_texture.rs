use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubmapTexture {
    pub cells: Vec<u8>,
    pub width: i32,
    pub height: i32,
    pub resolution: f64,
    pub slice_pose: crate::geometry_msgs::msg::Pose,
}

impl Default for SubmapTexture {
    fn default() -> Self {
        SubmapTexture {
            cells: Vec::new(),
            width: 0,
            height: 0,
            resolution: 0.0,
            slice_pose: crate::geometry_msgs::msg::Pose::default(),
        }
    }
}

impl ros2_client::Message for SubmapTexture {}
