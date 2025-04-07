use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetModelConfigurationRequest {
    pub model_name: ::std::string::String,
    pub urdf_param_name: ::std::string::String,
    pub joint_names: Vec<::std::string::String>,
    pub joint_positions: Vec<f64>,
}

impl Default for SetModelConfigurationRequest {
    fn default() -> Self {
        SetModelConfigurationRequest {
            model_name: ::std::string::String::new(),
            urdf_param_name: ::std::string::String::new(),
            joint_names: Vec::new(),
            joint_positions: Vec::new(),
        }
    }
}

impl ros2_client::Message for SetModelConfigurationRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetModelConfigurationResponse {
    pub success: bool,
    pub status_message: ::std::string::String,
}

impl Default for SetModelConfigurationResponse {
    fn default() -> Self {
        SetModelConfigurationResponse {
            success: false,
            status_message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SetModelConfigurationResponse {}


pub struct SetModelConfiguration;
impl ros2_client::Service for SetModelConfiguration {
    type Request = SetModelConfigurationRequest;
    type Response = SetModelConfigurationResponse;

    fn request_type_name(&self) -> &str { "SetModelConfigurationRequest" }
    fn response_type_name(&self) -> &str { "SetModelConfigurationResponse" }
}
