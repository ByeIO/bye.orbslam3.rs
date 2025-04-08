use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObjectColor {
    pub id: ::std::string::String,
    pub color: crate::std_msgs::msg::ColorRGBA,
}

impl Default for ObjectColor {
    fn default() -> Self {
        ObjectColor {
            id: ::std::string::String::new(),
            color: crate::std_msgs::msg::ColorRGBA::default(),
        }
    }
}

impl ros2_client::Message for ObjectColor {}
