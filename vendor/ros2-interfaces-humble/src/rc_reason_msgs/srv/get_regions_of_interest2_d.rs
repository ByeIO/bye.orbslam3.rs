use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetRegionsOfInterest2DRequest {
    pub region_of_interest_2d_ids: Vec<::std::string::String>,
}

impl Default for GetRegionsOfInterest2DRequest {
    fn default() -> Self {
        GetRegionsOfInterest2DRequest {
            region_of_interest_2d_ids: Vec::new(),
        }
    }
}

impl ros2_client::Message for GetRegionsOfInterest2DRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetRegionsOfInterest2DResponse {
    pub regions_of_interest: Vec<crate::rc_reason_msgs::msg::RegionOfInterest2D>,
    pub return_code: crate::rc_common_msgs::msg::ReturnCode,
}

impl Default for GetRegionsOfInterest2DResponse {
    fn default() -> Self {
        GetRegionsOfInterest2DResponse {
            regions_of_interest: Vec::new(),
            return_code: crate::rc_common_msgs::msg::ReturnCode::default(),
        }
    }
}

impl ros2_client::Message for GetRegionsOfInterest2DResponse {}


pub struct GetRegionsOfInterest2D;
impl ros2_client::Service for GetRegionsOfInterest2D {
    type Request = GetRegionsOfInterest2DRequest;
    type Response = GetRegionsOfInterest2DResponse;

    fn request_type_name(&self) -> &str { "GetRegionsOfInterest2DRequest" }
    fn response_type_name(&self) -> &str { "GetRegionsOfInterest2DResponse" }
}
