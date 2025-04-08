use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReloadDatabaseRequest {
    pub filepath: ::std::string::String,
}

impl Default for ReloadDatabaseRequest {
    fn default() -> Self {
        ReloadDatabaseRequest {
            filepath: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ReloadDatabaseRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReloadDatabaseResponse {
    pub success: bool,
}

impl Default for ReloadDatabaseResponse {
    fn default() -> Self {
        ReloadDatabaseResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for ReloadDatabaseResponse {}


pub struct ReloadDatabase;
impl ros2_client::Service for ReloadDatabase {
    type Request = ReloadDatabaseRequest;
    type Response = ReloadDatabaseResponse;

    fn request_type_name(&self) -> &str { "ReloadDatabaseRequest" }
    fn response_type_name(&self) -> &str { "ReloadDatabaseResponse" }
}
