use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReloadControllerLibrariesRequest {
    pub force_kill: bool,
}

impl Default for ReloadControllerLibrariesRequest {
    fn default() -> Self {
        ReloadControllerLibrariesRequest {
            force_kill: false,
        }
    }
}

impl ros2_client::Message for ReloadControllerLibrariesRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReloadControllerLibrariesResponse {
    pub ok: bool,
}

impl Default for ReloadControllerLibrariesResponse {
    fn default() -> Self {
        ReloadControllerLibrariesResponse {
            ok: false,
        }
    }
}

impl ros2_client::Message for ReloadControllerLibrariesResponse {}


pub struct ReloadControllerLibraries;
impl ros2_client::Service for ReloadControllerLibraries {
    type Request = ReloadControllerLibrariesRequest;
    type Response = ReloadControllerLibrariesResponse;

    fn request_type_name(&self) -> &str { "ReloadControllerLibrariesRequest" }
    fn response_type_name(&self) -> &str { "ReloadControllerLibrariesResponse" }
}
