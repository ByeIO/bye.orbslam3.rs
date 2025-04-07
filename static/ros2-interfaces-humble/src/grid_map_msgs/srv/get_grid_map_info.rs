use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetGridMapInfoRequest {

}

impl Default for GetGridMapInfoRequest {
    fn default() -> Self {
        GetGridMapInfoRequest {

        }
    }
}

impl ros2_client::Message for GetGridMapInfoRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetGridMapInfoResponse {
    pub info: crate::grid_map_msgs::msg::GridMapInfo,
}

impl Default for GetGridMapInfoResponse {
    fn default() -> Self {
        GetGridMapInfoResponse {
            info: crate::grid_map_msgs::msg::GridMapInfo::default(),
        }
    }
}

impl ros2_client::Message for GetGridMapInfoResponse {}


pub struct GetGridMapInfo;
impl ros2_client::Service for GetGridMapInfo {
    type Request = GetGridMapInfoRequest;
    type Response = GetGridMapInfoResponse;

    fn request_type_name(&self) -> &str { "GetGridMapInfoRequest" }
    fn response_type_name(&self) -> &str { "GetGridMapInfoResponse" }
}
