use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetRegionOfInterest3DRequest {
    pub region_of_interest: crate::rc_reason_msgs::msg::RegionOfInterest3D,
}

impl Default for SetRegionOfInterest3DRequest {
    fn default() -> Self {
        SetRegionOfInterest3DRequest {
            region_of_interest: crate::rc_reason_msgs::msg::RegionOfInterest3D::default(),
        }
    }
}

impl ros2_client::Message for SetRegionOfInterest3DRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetRegionOfInterest3DResponse {
    pub return_code: crate::rc_common_msgs::msg::ReturnCode,
}

impl Default for SetRegionOfInterest3DResponse {
    fn default() -> Self {
        SetRegionOfInterest3DResponse {
            return_code: crate::rc_common_msgs::msg::ReturnCode::default(),
        }
    }
}

impl ros2_client::Message for SetRegionOfInterest3DResponse {}


pub struct SetRegionOfInterest3D;
impl ros2_client::Service for SetRegionOfInterest3D {
    type Request = SetRegionOfInterest3DRequest;
    type Response = SetRegionOfInterest3DResponse;

    fn request_type_name(&self) -> &str { "SetRegionOfInterest3DRequest" }
    fn response_type_name(&self) -> &str { "SetRegionOfInterest3DResponse" }
}
