use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeatureFloatInfoGetRequest {
    pub feature_name: ::std::string::String,
    pub feature_module: crate::vimbax_camera_msgs::msg::FeatureModule,
}

impl Default for FeatureFloatInfoGetRequest {
    fn default() -> Self {
        FeatureFloatInfoGetRequest {
            feature_name: ::std::string::String::new(),
            feature_module: crate::vimbax_camera_msgs::msg::FeatureModule::default(),
        }
    }
}

impl ros2_client::Message for FeatureFloatInfoGetRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeatureFloatInfoGetResponse {
    pub min: f64,
    pub max: f64,
    pub inc: f64,
    pub inc_available: bool,
    pub error: crate::vimbax_camera_msgs::msg::Error,
}

impl Default for FeatureFloatInfoGetResponse {
    fn default() -> Self {
        FeatureFloatInfoGetResponse {
            min: 0.0,
            max: 0.0,
            inc: 0.0,
            inc_available: false,
            error: crate::vimbax_camera_msgs::msg::Error::default(),
        }
    }
}

impl ros2_client::Message for FeatureFloatInfoGetResponse {}


pub struct FeatureFloatInfoGet;
impl ros2_client::Service for FeatureFloatInfoGet {
    type Request = FeatureFloatInfoGetRequest;
    type Response = FeatureFloatInfoGetResponse;

    fn request_type_name(&self) -> &str { "FeatureFloatInfoGetRequest" }
    fn response_type_name(&self) -> &str { "FeatureFloatInfoGetResponse" }
}
