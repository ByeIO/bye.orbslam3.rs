use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ItemModel {
    #[serde(rename = "type")]    pub type_: ::std::string::String,
    pub unknown: crate::rc_reason_msgs::msg::RangeBox,
    pub rectangle: crate::rc_reason_msgs::msg::RangeRectangle,
}

impl ItemModel {
    pub const UNKNOWN: &'static str = "UNKNOWN";
    pub const RECTANGLE: &'static str = "RECTANGLE";
}

impl Default for ItemModel {
    fn default() -> Self {
        ItemModel {
            type_: ::std::string::String::new(),
            unknown: crate::rc_reason_msgs::msg::RangeBox::default(),
            rectangle: crate::rc_reason_msgs::msg::RangeRectangle::default(),
        }
    }
}

impl ros2_client::Message for ItemModel {}
