use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeatureRawInfoGetRequest {
    pub feature_name: ::std::string::String,
    pub feature_module: crate::vimbax_camera_msgs::msg::FeatureModule,
}

impl Default for FeatureRawInfoGetRequest {
    fn default() -> Self {
        FeatureRawInfoGetRequest {
            feature_name: ::std::string::String::new(),
            feature_module: crate::vimbax_camera_msgs::msg::FeatureModule::default(),
        }
    }
}

impl ros2_client::Message for FeatureRawInfoGetRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeatureRawInfoGetResponse {
    pub max_length: i64,
    pub error: crate::vimbax_camera_msgs::msg::Error,
}

impl Default for FeatureRawInfoGetResponse {
    fn default() -> Self {
        FeatureRawInfoGetResponse {
            max_length: 0,
            error: crate::vimbax_camera_msgs::msg::Error::default(),
        }
    }
}

impl ros2_client::Message for FeatureRawInfoGetResponse {}


pub struct FeatureRawInfoGet;
impl ros2_client::Service for FeatureRawInfoGet {
    type Request = FeatureRawInfoGetRequest;
    type Response = FeatureRawInfoGetResponse;

    fn request_type_name(&self) -> &str { "FeatureRawInfoGetRequest" }
    fn response_type_name(&self) -> &str { "FeatureRawInfoGetResponse" }
}
