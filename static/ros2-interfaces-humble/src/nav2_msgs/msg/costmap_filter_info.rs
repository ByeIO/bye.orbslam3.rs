use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CostmapFilterInfo {
    pub header: crate::std_msgs::msg::Header,
    #[serde(rename = "type")]    pub type_: u8,
    pub filter_mask_topic: ::std::string::String,
    pub base: f32,
    pub multiplier: f32,
}

impl Default for CostmapFilterInfo {
    fn default() -> Self {
        CostmapFilterInfo {
            header: crate::std_msgs::msg::Header::default(),
            type_: 0,
            filter_mask_topic: ::std::string::String::new(),
            base: 0.0,
            multiplier: 0.0,
        }
    }
}

impl ros2_client::Message for CostmapFilterInfo {}
