use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetWorldPropertiesRequest {

}

impl Default for GetWorldPropertiesRequest {
    fn default() -> Self {
        GetWorldPropertiesRequest {

        }
    }
}

impl ros2_client::Message for GetWorldPropertiesRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetWorldPropertiesResponse {
    pub sim_time: f64,
    pub model_names: Vec<::std::string::String>,
    pub rendering_enabled: bool,
    pub success: bool,
    pub status_message: ::std::string::String,
}

impl Default for GetWorldPropertiesResponse {
    fn default() -> Self {
        GetWorldPropertiesResponse {
            sim_time: 0.0,
            model_names: Vec::new(),
            rendering_enabled: false,
            success: false,
            status_message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetWorldPropertiesResponse {}


pub struct GetWorldProperties;
impl ros2_client::Service for GetWorldProperties {
    type Request = GetWorldPropertiesRequest;
    type Response = GetWorldPropertiesResponse;

    fn request_type_name(&self) -> &str { "GetWorldPropertiesRequest" }
    fn response_type_name(&self) -> &str { "GetWorldPropertiesResponse" }
}
