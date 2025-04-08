use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Detection2D {
    pub header: crate::std_msgs::msg::Header,
    pub results: Vec<crate::vision_msgs::msg::ObjectHypothesisWithPose>,
    pub bbox: crate::vision_msgs::msg::BoundingBox2D,
    pub id: ::std::string::String,
}

impl Default for Detection2D {
    fn default() -> Self {
        Detection2D {
            header: crate::std_msgs::msg::Header::default(),
            results: Vec::new(),
            bbox: crate::vision_msgs::msg::BoundingBox2D::default(),
            id: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for Detection2D {}
