use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetRegionOfInterest2DRequest {
    pub region_of_interest_2d: crate::rc_reason_msgs::msg::RegionOfInterest2D,
}

impl Default for SetRegionOfInterest2DRequest {
    fn default() -> Self {
        SetRegionOfInterest2DRequest {
            region_of_interest_2d: crate::rc_reason_msgs::msg::RegionOfInterest2D::default(),
        }
    }
}

impl ros2_client::Message for SetRegionOfInterest2DRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetRegionOfInterest2DResponse {
    pub return_code: crate::rc_common_msgs::msg::ReturnCode,
}

impl Default for SetRegionOfInterest2DResponse {
    fn default() -> Self {
        SetRegionOfInterest2DResponse {
            return_code: crate::rc_common_msgs::msg::ReturnCode::default(),
        }
    }
}

impl ros2_client::Message for SetRegionOfInterest2DResponse {}


pub struct SetRegionOfInterest2D;
impl ros2_client::Service for SetRegionOfInterest2D {
    type Request = SetRegionOfInterest2DRequest;
    type Response = SetRegionOfInterest2DResponse;

    fn request_type_name(&self) -> &str { "SetRegionOfInterest2DRequest" }
    fn response_type_name(&self) -> &str { "SetRegionOfInterest2DResponse" }
}
