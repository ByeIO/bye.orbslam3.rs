use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoadDatabaseRequest {
    pub database_path: ::std::string::String,
    pub clear: bool,
}

impl Default for LoadDatabaseRequest {
    fn default() -> Self {
        LoadDatabaseRequest {
            database_path: ::std::string::String::new(),
            clear: false,
        }
    }
}

impl ros2_client::Message for LoadDatabaseRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoadDatabaseResponse {

}

impl Default for LoadDatabaseResponse {
    fn default() -> Self {
        LoadDatabaseResponse {

        }
    }
}

impl ros2_client::Message for LoadDatabaseResponse {}


pub struct LoadDatabase;
impl ros2_client::Service for LoadDatabase {
    type Request = LoadDatabaseRequest;
    type Response = LoadDatabaseResponse;

    fn request_type_name(&self) -> &str { "LoadDatabaseRequest" }
    fn response_type_name(&self) -> &str { "LoadDatabaseResponse" }
}
