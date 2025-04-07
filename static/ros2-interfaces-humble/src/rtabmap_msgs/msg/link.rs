use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[serde_as]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Link {
    pub from_id: i32,
    pub to_id: i32,
    #[serde(rename = "type")]    pub type_: i32,
    pub transform: crate::geometry_msgs::msg::Transform,
    #[serde_as(as = "[_; 36]")]
    pub information: [f64; 36],
}

impl Default for Link {
    fn default() -> Self {
        Link {
            from_id: 0,
            to_id: 0,
            type_: 0,
            transform: crate::geometry_msgs::msg::Transform::default(),
            information: [0.0; 36],
        }
    }
}

impl ros2_client::Message for Link {}
