use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetGeozoneRequest {
    pub geozone: crate::as2_msgs::msg::Geozone,
}

impl Default for SetGeozoneRequest {
    fn default() -> Self {
        SetGeozoneRequest {
            geozone: crate::as2_msgs::msg::Geozone::default(),
        }
    }
}

impl ros2_client::Message for SetGeozoneRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetGeozoneResponse {
    pub success: bool,
}

impl Default for SetGeozoneResponse {
    fn default() -> Self {
        SetGeozoneResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for SetGeozoneResponse {}


pub struct SetGeozone;
impl ros2_client::Service for SetGeozone {
    type Request = SetGeozoneRequest;
    type Response = SetGeozoneResponse;

    fn request_type_name(&self) -> &str { "SetGeozoneRequest" }
    fn response_type_name(&self) -> &str { "SetGeozoneResponse" }
}
