use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConfigLoggerRequest {
    pub logger_name: ::std::string::String,
    pub level: ::std::string::String,
}

impl Default for ConfigLoggerRequest {
    fn default() -> Self {
        ConfigLoggerRequest {
            logger_name: ::std::string::String::new(),
            level: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ConfigLoggerRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConfigLoggerResponse {
    pub success: bool,
}

impl Default for ConfigLoggerResponse {
    fn default() -> Self {
        ConfigLoggerResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for ConfigLoggerResponse {}


pub struct ConfigLogger;
impl ros2_client::Service for ConfigLogger {
    type Request = ConfigLoggerRequest;
    type Response = ConfigLoggerResponse;

    fn request_type_name(&self) -> &str { "ConfigLoggerRequest" }
    fn response_type_name(&self) -> &str { "ConfigLoggerResponse" }
}
