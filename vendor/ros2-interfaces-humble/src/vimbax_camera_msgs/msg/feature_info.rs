use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeatureInfo {
    pub name: ::std::string::String,
    pub category: ::std::string::String,
    pub display_name: ::std::string::String,
    pub sfnc_namespace: ::std::string::String,
    pub unit: ::std::string::String,
    pub data_type: u32,
    pub flags: crate::vimbax_camera_msgs::msg::FeatureFlags,
    pub polling_time: u32,
}

impl Default for FeatureInfo {
    fn default() -> Self {
        FeatureInfo {
            name: ::std::string::String::new(),
            category: ::std::string::String::new(),
            display_name: ::std::string::String::new(),
            sfnc_namespace: ::std::string::String::new(),
            unit: ::std::string::String::new(),
            data_type: 0,
            flags: crate::vimbax_camera_msgs::msg::FeatureFlags::default(),
            polling_time: 0,
        }
    }
}

impl ros2_client::Message for FeatureInfo {}
