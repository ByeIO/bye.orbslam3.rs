use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DetectLoadCarriersRequest {
    pub pose_frame: ::std::string::String,
    pub region_of_interest_id: ::std::string::String,
    pub region_of_interest_2d_id: ::std::string::String,
    pub load_carrier_ids: Vec<::std::string::String>,
    pub robot_pose: crate::geometry_msgs::msg::Pose,
}

impl Default for DetectLoadCarriersRequest {
    fn default() -> Self {
        DetectLoadCarriersRequest {
            pose_frame: ::std::string::String::new(),
            region_of_interest_id: ::std::string::String::new(),
            region_of_interest_2d_id: ::std::string::String::new(),
            load_carrier_ids: Vec::new(),
            robot_pose: crate::geometry_msgs::msg::Pose::default(),
        }
    }
}

impl ros2_client::Message for DetectLoadCarriersRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DetectLoadCarriersResponse {
    pub timestamp: crate::builtin_interfaces::msg::Time,
    pub load_carriers: Vec<crate::rc_reason_msgs::msg::LoadCarrier>,
    pub return_code: crate::rc_common_msgs::msg::ReturnCode,
}

impl Default for DetectLoadCarriersResponse {
    fn default() -> Self {
        DetectLoadCarriersResponse {
            timestamp: crate::builtin_interfaces::msg::Time::default(),
            load_carriers: Vec::new(),
            return_code: crate::rc_common_msgs::msg::ReturnCode::default(),
        }
    }
}

impl ros2_client::Message for DetectLoadCarriersResponse {}


pub struct DetectLoadCarriers;
impl ros2_client::Service for DetectLoadCarriers {
    type Request = DetectLoadCarriersRequest;
    type Response = DetectLoadCarriersResponse;

    fn request_type_name(&self) -> &str { "DetectLoadCarriersRequest" }
    fn response_type_name(&self) -> &str { "DetectLoadCarriersResponse" }
}
