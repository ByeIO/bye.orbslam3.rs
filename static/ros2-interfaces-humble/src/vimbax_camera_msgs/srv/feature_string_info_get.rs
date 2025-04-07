use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeatureStringInfoGetRequest {
    pub feature_name: ::std::string::String,
    pub feature_module: crate::vimbax_camera_msgs::msg::FeatureModule,
}

impl Default for FeatureStringInfoGetRequest {
    fn default() -> Self {
        FeatureStringInfoGetRequest {
            feature_name: ::std::string::String::new(),
            feature_module: crate::vimbax_camera_msgs::msg::FeatureModule::default(),
        }
    }
}

impl ros2_client::Message for FeatureStringInfoGetRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeatureStringInfoGetResponse {
    pub max_length: i64,
    pub error: crate::vimbax_camera_msgs::msg::Error,
}

impl Default for FeatureStringInfoGetResponse {
    fn default() -> Self {
        FeatureStringInfoGetResponse {
            max_length: 0,
            error: crate::vimbax_camera_msgs::msg::Error::default(),
        }
    }
}

impl ros2_client::Message for FeatureStringInfoGetResponse {}


pub struct FeatureStringInfoGet;
impl ros2_client::Service for FeatureStringInfoGet {
    type Request = FeatureStringInfoGetRequest;
    type Response = FeatureStringInfoGetResponse;

    fn request_type_name(&self) -> &str { "FeatureStringInfoGetRequest" }
    fn response_type_name(&self) -> &str { "FeatureStringInfoGetResponse" }
}
