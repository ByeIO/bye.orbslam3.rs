use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetRobotSoftwareVersionRequest {

}

impl Default for GetRobotSoftwareVersionRequest {
    fn default() -> Self {
        GetRobotSoftwareVersionRequest {

        }
    }
}

impl ros2_client::Message for GetRobotSoftwareVersionRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetRobotSoftwareVersionResponse {
    pub major: u32,
    pub minor: u32,
    pub bugfix: u32,
    pub build: u32,
}

impl Default for GetRobotSoftwareVersionResponse {
    fn default() -> Self {
        GetRobotSoftwareVersionResponse {
            major: 0,
            minor: 0,
            bugfix: 0,
            build: 0,
        }
    }
}

impl ros2_client::Message for GetRobotSoftwareVersionResponse {}


pub struct GetRobotSoftwareVersion;
impl ros2_client::Service for GetRobotSoftwareVersion {
    type Request = GetRobotSoftwareVersionRequest;
    type Response = GetRobotSoftwareVersionResponse;

    fn request_type_name(&self) -> &str { "GetRobotSoftwareVersionRequest" }
    fn response_type_name(&self) -> &str { "GetRobotSoftwareVersionResponse" }
}
