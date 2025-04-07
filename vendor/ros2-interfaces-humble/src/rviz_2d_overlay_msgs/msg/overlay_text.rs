use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OverlayText {
    pub action: u8,
    pub width: i32,
    pub height: i32,
    pub horizontal_distance: i32,
    pub vertical_distance: i32,
    pub horizontal_alignment: u8,
    pub vertical_alignment: u8,
    pub bg_color: crate::std_msgs::msg::ColorRGBA,
    pub line_width: i32,
    pub text_size: f32,
    pub font: ::std::string::String,
    pub fg_color: crate::std_msgs::msg::ColorRGBA,
    pub text: ::std::string::String,
}

impl OverlayText {
    pub const ADD: u8 = 0;
    pub const DELETE: u8 = 1;
    pub const LEFT: u8 = 0;
    pub const RIGHT: u8 = 1;
    pub const CENTER: u8 = 2;
    pub const TOP: u8 = 3;
    pub const BOTTOM: u8 = 4;
}

impl Default for OverlayText {
    fn default() -> Self {
        OverlayText {
            action: 0,
            width: 0,
            height: 0,
            horizontal_distance: 0,
            vertical_distance: 0,
            horizontal_alignment: 0,
            vertical_alignment: 0,
            bg_color: crate::std_msgs::msg::ColorRGBA::default(),
            line_width: 0,
            text_size: 0.0,
            font: ::std::string::String::new(),
            fg_color: crate::std_msgs::msg::ColorRGBA::default(),
            text: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for OverlayText {}
