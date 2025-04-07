use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DetectItemsRequest {
    pub pose_frame: ::std::string::String,
    pub region_of_interest_id: ::std::string::String,
    pub load_carrier_id: ::std::string::String,
    pub load_carrier_compartment: crate::rc_reason_msgs::msg::Compartment,
    pub item_models: Vec<crate::rc_reason_msgs::msg::ItemModel>,
    pub robot_pose: crate::geometry_msgs::msg::Pose,
}

impl Default for DetectItemsRequest {
    fn default() -> Self {
        DetectItemsRequest {
            pose_frame: ::std::string::String::new(),
            region_of_interest_id: ::std::string::String::new(),
            load_carrier_id: ::std::string::String::new(),
            load_carrier_compartment: crate::rc_reason_msgs::msg::Compartment::default(),
            item_models: Vec::new(),
            robot_pose: crate::geometry_msgs::msg::Pose::default(),
        }
    }
}

impl ros2_client::Message for DetectItemsRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DetectItemsResponse {
    pub timestamp: crate::builtin_interfaces::msg::Time,
    pub items: Vec<crate::rc_reason_msgs::msg::Item>,
    pub load_carriers: Vec<crate::rc_reason_msgs::msg::LoadCarrier>,
    pub return_code: crate::rc_common_msgs::msg::ReturnCode,
}

impl Default for DetectItemsResponse {
    fn default() -> Self {
        DetectItemsResponse {
            timestamp: crate::builtin_interfaces::msg::Time::default(),
            items: Vec::new(),
            load_carriers: Vec::new(),
            return_code: crate::rc_common_msgs::msg::ReturnCode::default(),
        }
    }
}

impl ros2_client::Message for DetectItemsResponse {}


pub struct DetectItems;
impl ros2_client::Service for DetectItems {
    type Request = DetectItemsRequest;
    type Response = DetectItemsResponse;

    fn request_type_name(&self) -> &str { "DetectItemsRequest" }
    fn response_type_name(&self) -> &str { "DetectItemsResponse" }
}
