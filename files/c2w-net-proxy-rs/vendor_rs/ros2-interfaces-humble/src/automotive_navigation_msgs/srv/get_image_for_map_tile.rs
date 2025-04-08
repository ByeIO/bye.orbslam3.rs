use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetImageForMapTileRequest {
    pub tile_id: ::std::string::String,
}

impl Default for GetImageForMapTileRequest {
    fn default() -> Self {
        GetImageForMapTileRequest {
            tile_id: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetImageForMapTileRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetImageForMapTileResponse {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for GetImageForMapTileResponse {
    fn default() -> Self {
        GetImageForMapTileResponse {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetImageForMapTileResponse {}


pub struct GetImageForMapTile;
impl ros2_client::Service for GetImageForMapTile {
    type Request = GetImageForMapTileRequest;
    type Response = GetImageForMapTileResponse;

    fn request_type_name(&self) -> &str { "GetImageForMapTileRequest" }
    fn response_type_name(&self) -> &str { "GetImageForMapTileResponse" }
}
