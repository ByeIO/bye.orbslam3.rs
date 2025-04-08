use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TsrVisionOnly {
    pub header: crate::std_msgs::msg::Header,
    pub vision_only_sign_type_display1: u8,
    pub vision_only_supplementary_sign_type_display1: u8,
    pub vision_only_sign_type_display2: u8,
    pub vision_only_supplementary_sign_type_display2: u8,
    pub vision_only_sign_type_display3: u8,
    pub vision_only_supplementary_sign_type_display3: u8,
    pub vision_only_sign_type_display4: u8,
    pub vision_only_supplementary_sign_type_display4: u8,
}

impl Default for TsrVisionOnly {
    fn default() -> Self {
        TsrVisionOnly {
            header: crate::std_msgs::msg::Header::default(),
            vision_only_sign_type_display1: 0,
            vision_only_supplementary_sign_type_display1: 0,
            vision_only_sign_type_display2: 0,
            vision_only_supplementary_sign_type_display2: 0,
            vision_only_sign_type_display3: 0,
            vision_only_supplementary_sign_type_display3: 0,
            vision_only_sign_type_display4: 0,
            vision_only_supplementary_sign_type_display4: 0,
        }
    }
}

impl ros2_client::Message for TsrVisionOnly {}
