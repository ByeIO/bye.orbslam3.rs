use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetLightPropertiesRequest {
    pub light_name: ::std::string::String,
    pub diffuse: crate::std_msgs::msg::ColorRGBA,
    pub attenuation_constant: f64,
    pub attenuation_linear: f64,
    pub attenuation_quadratic: f64,
}

impl Default for SetLightPropertiesRequest {
    fn default() -> Self {
        SetLightPropertiesRequest {
            light_name: ::std::string::String::new(),
            diffuse: crate::std_msgs::msg::ColorRGBA::default(),
            attenuation_constant: 0.0,
            attenuation_linear: 0.0,
            attenuation_quadratic: 0.0,
        }
    }
}

impl ros2_client::Message for SetLightPropertiesRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetLightPropertiesResponse {
    pub success: bool,
    pub status_message: ::std::string::String,
}

impl Default for SetLightPropertiesResponse {
    fn default() -> Self {
        SetLightPropertiesResponse {
            success: false,
            status_message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SetLightPropertiesResponse {}


pub struct SetLightProperties;
impl ros2_client::Service for SetLightProperties {
    type Request = SetLightPropertiesRequest;
    type Response = SetLightPropertiesResponse;

    fn request_type_name(&self) -> &str { "SetLightPropertiesRequest" }
    fn response_type_name(&self) -> &str { "SetLightPropertiesResponse" }
}
