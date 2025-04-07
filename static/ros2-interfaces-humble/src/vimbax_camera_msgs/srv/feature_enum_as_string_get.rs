use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeatureEnumAsStringGetRequest {
    pub feature_name: ::std::string::String,
    pub feature_module: crate::vimbax_camera_msgs::msg::FeatureModule,
    pub value: i64,
}

impl Default for FeatureEnumAsStringGetRequest {
    fn default() -> Self {
        FeatureEnumAsStringGetRequest {
            feature_name: ::std::string::String::new(),
            feature_module: crate::vimbax_camera_msgs::msg::FeatureModule::default(),
            value: 0,
        }
    }
}

impl ros2_client::Message for FeatureEnumAsStringGetRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeatureEnumAsStringGetResponse {
    pub option: ::std::string::String,
    pub error: crate::vimbax_camera_msgs::msg::Error,
}

impl Default for FeatureEnumAsStringGetResponse {
    fn default() -> Self {
        FeatureEnumAsStringGetResponse {
            option: ::std::string::String::new(),
            error: crate::vimbax_camera_msgs::msg::Error::default(),
        }
    }
}

impl ros2_client::Message for FeatureEnumAsStringGetResponse {}


pub struct FeatureEnumAsStringGet;
impl ros2_client::Service for FeatureEnumAsStringGet {
    type Request = FeatureEnumAsStringGetRequest;
    type Response = FeatureEnumAsStringGetResponse;

    fn request_type_name(&self) -> &str { "FeatureEnumAsStringGetRequest" }
    fn response_type_name(&self) -> &str { "FeatureEnumAsStringGetResponse" }
}
