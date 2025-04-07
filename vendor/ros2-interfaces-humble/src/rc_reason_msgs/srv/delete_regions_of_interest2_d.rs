use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteRegionsOfInterest2DRequest {
    pub region_of_interest_2d_ids: Vec<::std::string::String>,
}

impl Default for DeleteRegionsOfInterest2DRequest {
    fn default() -> Self {
        DeleteRegionsOfInterest2DRequest {
            region_of_interest_2d_ids: Vec::new(),
        }
    }
}

impl ros2_client::Message for DeleteRegionsOfInterest2DRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteRegionsOfInterest2DResponse {
    pub return_code: crate::rc_common_msgs::msg::ReturnCode,
}

impl Default for DeleteRegionsOfInterest2DResponse {
    fn default() -> Self {
        DeleteRegionsOfInterest2DResponse {
            return_code: crate::rc_common_msgs::msg::ReturnCode::default(),
        }
    }
}

impl ros2_client::Message for DeleteRegionsOfInterest2DResponse {}


pub struct DeleteRegionsOfInterest2D;
impl ros2_client::Service for DeleteRegionsOfInterest2D {
    type Request = DeleteRegionsOfInterest2DRequest;
    type Response = DeleteRegionsOfInterest2DResponse;

    fn request_type_name(&self) -> &str { "DeleteRegionsOfInterest2DRequest" }
    fn response_type_name(&self) -> &str { "DeleteRegionsOfInterest2DResponse" }
}
