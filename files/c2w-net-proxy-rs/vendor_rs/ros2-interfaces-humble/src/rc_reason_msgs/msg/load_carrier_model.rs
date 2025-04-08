use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoadCarrierModel {
    pub id: ::std::string::String,
    #[serde(rename = "type")]    pub type_: ::std::string::String,
    pub outer_dimensions: crate::rc_reason_msgs::msg::Box,
    pub inner_dimensions: crate::rc_reason_msgs::msg::Box,
    pub rim_thickness: crate::rc_reason_msgs::msg::Rectangle,
    pub rim_step_height: f64,
    pub rim_ledge: crate::rc_reason_msgs::msg::Rectangle,
    pub height_open_side: f64,
    pub pose: crate::geometry_msgs::msg::PoseStamped,
    pub pose_type: ::std::string::String,
}

impl Default for LoadCarrierModel {
    fn default() -> Self {
        LoadCarrierModel {
            id: ::std::string::String::new(),
            type_: ::std::string::String::new(),
            outer_dimensions: crate::rc_reason_msgs::msg::Box::default(),
            inner_dimensions: crate::rc_reason_msgs::msg::Box::default(),
            rim_thickness: crate::rc_reason_msgs::msg::Rectangle::default(),
            rim_step_height: 0.0,
            rim_ledge: crate::rc_reason_msgs::msg::Rectangle::default(),
            height_open_side: 0.0,
            pose: crate::geometry_msgs::msg::PoseStamped::default(),
            pose_type: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for LoadCarrierModel {}
