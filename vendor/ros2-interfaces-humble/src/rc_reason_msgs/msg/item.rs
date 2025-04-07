use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Item {
    pub uuid: ::std::string::String,
    pub grasp_uuids: Vec<::std::string::String>,
    #[serde(rename = "type")]    pub type_: ::std::string::String,
    pub rectangle: crate::rc_reason_msgs::msg::Rectangle,
    pub pose: crate::geometry_msgs::msg::PoseStamped,
}

impl Item {
    pub const RECTANGLE: &'static str = "RECTANGLE";
}

impl Default for Item {
    fn default() -> Self {
        Item {
            uuid: ::std::string::String::new(),
            grasp_uuids: Vec::new(),
            type_: ::std::string::String::new(),
            rectangle: crate::rc_reason_msgs::msg::Rectangle::default(),
            pose: crate::geometry_msgs::msg::PoseStamped::default(),
        }
    }
}

impl ros2_client::Message for Item {}
