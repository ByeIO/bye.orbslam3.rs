use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeatureEnumGetRequest {
    pub feature_name: ::std::string::String,
    pub feature_module: crate::vimbax_camera_msgs::msg::FeatureModule,
}

impl Default for FeatureEnumGetRequest {
    fn default() -> Self {
        FeatureEnumGetRequest {
            feature_name: ::std::string::String::new(),
            feature_module: crate::vimbax_camera_msgs::msg::FeatureModule::default(),
        }
    }
}

impl ros2_client::Message for FeatureEnumGetRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeatureEnumGetResponse {
    pub value: ::std::string::String,
    pub error: crate::vimbax_camera_msgs::msg::Error,
}

impl Default for FeatureEnumGetResponse {
    fn default() -> Self {
        FeatureEnumGetResponse {
            value: ::std::string::String::new(),
            error: crate::vimbax_camera_msgs::msg::Error::default(),
        }
    }
}

impl ros2_client::Message for FeatureEnumGetResponse {}


pub struct FeatureEnumGet;
impl ros2_client::Service for FeatureEnumGet {
    type Request = FeatureEnumGetRequest;
    type Response = FeatureEnumGetResponse;

    fn request_type_name(&self) -> &str { "FeatureEnumGetRequest" }
    fn response_type_name(&self) -> &str { "FeatureEnumGetResponse" }
}
