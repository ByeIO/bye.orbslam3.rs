use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Signal {
    pub header: crate::std_msgs::msg::Header,
    pub id: u32,
    pub label: ::std::string::String,
    pub score: f32,
    pub bbox: crate::lgsvl_msgs::msg::BoundingBox3D,
}

impl Default for Signal {
    fn default() -> Self {
        Signal {
            header: crate::std_msgs::msg::Header::default(),
            id: 0,
            label: ::std::string::String::new(),
            score: 0.0,
            bbox: crate::lgsvl_msgs::msg::BoundingBox3D::default(),
        }
    }
}

impl ros2_client::Message for Signal {}
