use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetLightPropertiesRequest {
    pub light_name: ::std::string::String,
}

impl Default for GetLightPropertiesRequest {
    fn default() -> Self {
        GetLightPropertiesRequest {
            light_name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetLightPropertiesRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetLightPropertiesResponse {
    pub diffuse: crate::std_msgs::msg::ColorRGBA,
    pub attenuation_constant: f64,
    pub attenuation_linear: f64,
    pub attenuation_quadratic: f64,
    pub success: bool,
    pub status_message: ::std::string::String,
}

impl Default for GetLightPropertiesResponse {
    fn default() -> Self {
        GetLightPropertiesResponse {
            diffuse: crate::std_msgs::msg::ColorRGBA::default(),
            attenuation_constant: 0.0,
            attenuation_linear: 0.0,
            attenuation_quadratic: 0.0,
            success: false,
            status_message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetLightPropertiesResponse {}


pub struct GetLightProperties;
impl ros2_client::Service for GetLightProperties {
    type Request = GetLightPropertiesRequest;
    type Response = GetLightPropertiesResponse;

    fn request_type_name(&self) -> &str { "GetLightPropertiesRequest" }
    fn response_type_name(&self) -> &str { "GetLightPropertiesResponse" }
}
