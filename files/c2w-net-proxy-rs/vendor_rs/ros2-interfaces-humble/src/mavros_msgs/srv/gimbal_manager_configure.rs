use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GimbalManagerConfigureRequest {
    pub sysid_primary: i16,
    pub compid_primary: i16,
    pub sysid_secondary: i16,
    pub compid_secondary: i16,
    pub gimbal_device_id: u8,
}

impl Default for GimbalManagerConfigureRequest {
    fn default() -> Self {
        GimbalManagerConfigureRequest {
            sysid_primary: 0,
            compid_primary: 0,
            sysid_secondary: 0,
            compid_secondary: 0,
            gimbal_device_id: 0,
        }
    }
}

impl ros2_client::Message for GimbalManagerConfigureRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GimbalManagerConfigureResponse {
    pub success: bool,
    pub result: u8,
}

impl Default for GimbalManagerConfigureResponse {
    fn default() -> Self {
        GimbalManagerConfigureResponse {
            success: false,
            result: 0,
        }
    }
}

impl ros2_client::Message for GimbalManagerConfigureResponse {}


pub struct GimbalManagerConfigure;
impl ros2_client::Service for GimbalManagerConfigure {
    type Request = GimbalManagerConfigureRequest;
    type Response = GimbalManagerConfigureResponse;

    fn request_type_name(&self) -> &str { "GimbalManagerConfigureRequest" }
    fn response_type_name(&self) -> &str { "GimbalManagerConfigureResponse" }
}
