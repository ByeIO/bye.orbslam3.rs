use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PointOfInterestResponse {
    pub header: crate::std_msgs::msg::Header,
    pub name: ::std::string::String,
    pub module_name: ::std::string::String,
    pub request_id: u16,
    pub update_num: u16,
    pub point_statuses: Vec<crate::automotive_navigation_msgs::msg::PointOfInterestStatus>,
}

impl Default for PointOfInterestResponse {
    fn default() -> Self {
        PointOfInterestResponse {
            header: crate::std_msgs::msg::Header::default(),
            name: ::std::string::String::new(),
            module_name: ::std::string::String::new(),
            request_id: 0,
            update_num: 0,
            point_statuses: Vec::new(),
        }
    }
}

impl ros2_client::Message for PointOfInterestResponse {}
