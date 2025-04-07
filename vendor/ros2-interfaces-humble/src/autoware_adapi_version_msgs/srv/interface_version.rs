use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InterfaceVersionRequest {

}

impl Default for InterfaceVersionRequest {
    fn default() -> Self {
        InterfaceVersionRequest {

        }
    }
}

impl ros2_client::Message for InterfaceVersionRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InterfaceVersionResponse {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
}

impl Default for InterfaceVersionResponse {
    fn default() -> Self {
        InterfaceVersionResponse {
            major: 0,
            minor: 0,
            patch: 0,
        }
    }
}

impl ros2_client::Message for InterfaceVersionResponse {}


pub struct InterfaceVersion;
impl ros2_client::Service for InterfaceVersion {
    type Request = InterfaceVersionRequest;
    type Response = InterfaceVersionResponse;

    fn request_type_name(&self) -> &str { "InterfaceVersionRequest" }
    fn response_type_name(&self) -> &str { "InterfaceVersionResponse" }
}
