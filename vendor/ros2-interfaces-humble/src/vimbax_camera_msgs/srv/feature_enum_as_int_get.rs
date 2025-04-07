use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeatureEnumAsIntGetRequest {
    pub feature_name: ::std::string::String,
    pub feature_module: crate::vimbax_camera_msgs::msg::FeatureModule,
    pub option: ::std::string::String,
}

impl Default for FeatureEnumAsIntGetRequest {
    fn default() -> Self {
        FeatureEnumAsIntGetRequest {
            feature_name: ::std::string::String::new(),
            feature_module: crate::vimbax_camera_msgs::msg::FeatureModule::default(),
            option: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for FeatureEnumAsIntGetRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeatureEnumAsIntGetResponse {
    pub value: i64,
    pub error: crate::vimbax_camera_msgs::msg::Error,
}

impl Default for FeatureEnumAsIntGetResponse {
    fn default() -> Self {
        FeatureEnumAsIntGetResponse {
            value: 0,
            error: crate::vimbax_camera_msgs::msg::Error::default(),
        }
    }
}

impl ros2_client::Message for FeatureEnumAsIntGetResponse {}


pub struct FeatureEnumAsIntGet;
impl ros2_client::Service for FeatureEnumAsIntGet {
    type Request = FeatureEnumAsIntGetRequest;
    type Response = FeatureEnumAsIntGetResponse;

    fn request_type_name(&self) -> &str { "FeatureEnumAsIntGetRequest" }
    fn response_type_name(&self) -> &str { "FeatureEnumAsIntGetResponse" }
}
