use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetRegionsOfInterest3DRequest {
    pub region_of_interest_ids: Vec<::std::string::String>,
}

impl Default for GetRegionsOfInterest3DRequest {
    fn default() -> Self {
        GetRegionsOfInterest3DRequest {
            region_of_interest_ids: Vec::new(),
        }
    }
}

impl ros2_client::Message for GetRegionsOfInterest3DRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetRegionsOfInterest3DResponse {
    pub regions_of_interest: Vec<crate::rc_reason_msgs::msg::RegionOfInterest3D>,
    pub return_code: crate::rc_common_msgs::msg::ReturnCode,
}

impl Default for GetRegionsOfInterest3DResponse {
    fn default() -> Self {
        GetRegionsOfInterest3DResponse {
            regions_of_interest: Vec::new(),
            return_code: crate::rc_common_msgs::msg::ReturnCode::default(),
        }
    }
}

impl ros2_client::Message for GetRegionsOfInterest3DResponse {}


pub struct GetRegionsOfInterest3D;
impl ros2_client::Service for GetRegionsOfInterest3D {
    type Request = GetRegionsOfInterest3DRequest;
    type Response = GetRegionsOfInterest3DResponse;

    fn request_type_name(&self) -> &str { "GetRegionsOfInterest3DRequest" }
    fn response_type_name(&self) -> &str { "GetRegionsOfInterest3DResponse" }
}
