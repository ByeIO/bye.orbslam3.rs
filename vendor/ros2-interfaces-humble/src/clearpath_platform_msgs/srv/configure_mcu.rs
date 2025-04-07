use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConfigureMcuRequest {
    pub domain_id: u8,
    pub robot_namespace: ::std::string::String,
}

impl Default for ConfigureMcuRequest {
    fn default() -> Self {
        ConfigureMcuRequest {
            domain_id: 0,
            robot_namespace: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ConfigureMcuRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConfigureMcuResponse {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for ConfigureMcuResponse {
    fn default() -> Self {
        ConfigureMcuResponse {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ConfigureMcuResponse {}


pub struct ConfigureMcu;
impl ros2_client::Service for ConfigureMcu {
    type Request = ConfigureMcuRequest;
    type Response = ConfigureMcuResponse;

    fn request_type_name(&self) -> &str { "ConfigureMcuRequest" }
    fn response_type_name(&self) -> &str { "ConfigureMcuResponse" }
}
