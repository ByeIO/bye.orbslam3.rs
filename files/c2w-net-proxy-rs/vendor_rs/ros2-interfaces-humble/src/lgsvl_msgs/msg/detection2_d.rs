use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Detection2D {
    pub header: crate::std_msgs::msg::Header,
    pub id: u32,
    pub label: ::std::string::String,
    pub score: f32,
    pub bbox: crate::lgsvl_msgs::msg::BoundingBox2D,
    pub velocity: crate::geometry_msgs::msg::Twist,
}

impl Default for Detection2D {
    fn default() -> Self {
        Detection2D {
            header: crate::std_msgs::msg::Header::default(),
            id: 0,
            label: ::std::string::String::new(),
            score: 0.0,
            bbox: crate::lgsvl_msgs::msg::BoundingBox2D::default(),
            velocity: crate::geometry_msgs::msg::Twist::default(),
        }
    }
}

impl ros2_client::Message for Detection2D {}
