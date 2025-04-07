use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PointOfInterestRequest {
    pub header: crate::std_msgs::msg::Header,
    pub name: ::std::string::String,
    pub module_name: ::std::string::String,
    pub request_id: u16,
    pub cancel: u16,
    pub update_num: u16,
    pub guid_valid: u16,
    pub guid: u64,
    pub tolerance: f32,
}

impl Default for PointOfInterestRequest {
    fn default() -> Self {
        PointOfInterestRequest {
            header: crate::std_msgs::msg::Header::default(),
            name: ::std::string::String::new(),
            module_name: ::std::string::String::new(),
            request_id: 0,
            cancel: 0,
            update_num: 0,
            guid_valid: 0,
            guid: 0,
            tolerance: 0.0,
        }
    }
}

impl ros2_client::Message for PointOfInterestRequest {}
