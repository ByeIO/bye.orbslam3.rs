use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeatureIntInfoGetRequest {
    pub feature_name: ::std::string::String,
    pub feature_module: crate::vimbax_camera_msgs::msg::FeatureModule,
}

impl Default for FeatureIntInfoGetRequest {
    fn default() -> Self {
        FeatureIntInfoGetRequest {
            feature_name: ::std::string::String::new(),
            feature_module: crate::vimbax_camera_msgs::msg::FeatureModule::default(),
        }
    }
}

impl ros2_client::Message for FeatureIntInfoGetRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeatureIntInfoGetResponse {
    pub min: i64,
    pub max: i64,
    pub inc: i64,
    pub error: crate::vimbax_camera_msgs::msg::Error,
}

impl Default for FeatureIntInfoGetResponse {
    fn default() -> Self {
        FeatureIntInfoGetResponse {
            min: 0,
            max: 0,
            inc: 0,
            error: crate::vimbax_camera_msgs::msg::Error::default(),
        }
    }
}

impl ros2_client::Message for FeatureIntInfoGetResponse {}


pub struct FeatureIntInfoGet;
impl ros2_client::Service for FeatureIntInfoGet {
    type Request = FeatureIntInfoGetRequest;
    type Response = FeatureIntInfoGetResponse;

    fn request_type_name(&self) -> &str { "FeatureIntInfoGetRequest" }
    fn response_type_name(&self) -> &str { "FeatureIntInfoGetResponse" }
}
