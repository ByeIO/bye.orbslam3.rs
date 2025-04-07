use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetGeozoneRequest {

}

impl Default for GetGeozoneRequest {
    fn default() -> Self {
        GetGeozoneRequest {

        }
    }
}

impl ros2_client::Message for GetGeozoneRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetGeozoneResponse {
    pub success: bool,
    pub geozone_list: Vec<crate::as2_msgs::msg::Geozone>,
}

impl Default for GetGeozoneResponse {
    fn default() -> Self {
        GetGeozoneResponse {
            success: false,
            geozone_list: Vec::new(),
        }
    }
}

impl ros2_client::Message for GetGeozoneResponse {}


pub struct GetGeozone;
impl ros2_client::Service for GetGeozone {
    type Request = GetGeozoneRequest;
    type Response = GetGeozoneResponse;

    fn request_type_name(&self) -> &str { "GetGeozoneRequest" }
    fn response_type_name(&self) -> &str { "GetGeozoneResponse" }
}
