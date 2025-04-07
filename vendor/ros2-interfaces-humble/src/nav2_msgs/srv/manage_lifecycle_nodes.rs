use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ManageLifecycleNodesRequest {
    pub command: u8,
}

impl ManageLifecycleNodesRequest {
    pub const STARTUP: u8 = 0;
    pub const PAUSE: u8 = 1;
    pub const RESUME: u8 = 2;
    pub const RESET: u8 = 3;
    pub const SHUTDOWN: u8 = 4;
}

impl Default for ManageLifecycleNodesRequest {
    fn default() -> Self {
        ManageLifecycleNodesRequest {
            command: 0,
        }
    }
}

impl ros2_client::Message for ManageLifecycleNodesRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ManageLifecycleNodesResponse {
    pub success: bool,
}

impl Default for ManageLifecycleNodesResponse {
    fn default() -> Self {
        ManageLifecycleNodesResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for ManageLifecycleNodesResponse {}


pub struct ManageLifecycleNodes;
impl ros2_client::Service for ManageLifecycleNodes {
    type Request = ManageLifecycleNodesRequest;
    type Response = ManageLifecycleNodesResponse;

    fn request_type_name(&self) -> &str { "ManageLifecycleNodesRequest" }
    fn response_type_name(&self) -> &str { "ManageLifecycleNodesResponse" }
}
