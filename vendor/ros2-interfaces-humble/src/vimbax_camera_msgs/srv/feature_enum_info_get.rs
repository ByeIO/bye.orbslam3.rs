use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeatureEnumInfoGetRequest {
    pub feature_name: ::std::string::String,
    pub feature_module: crate::vimbax_camera_msgs::msg::FeatureModule,
}

impl Default for FeatureEnumInfoGetRequest {
    fn default() -> Self {
        FeatureEnumInfoGetRequest {
            feature_name: ::std::string::String::new(),
            feature_module: crate::vimbax_camera_msgs::msg::FeatureModule::default(),
        }
    }
}

impl ros2_client::Message for FeatureEnumInfoGetRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeatureEnumInfoGetResponse {
    pub possible_values: Vec<::std::string::String>,
    pub available_values: Vec<::std::string::String>,
    pub error: crate::vimbax_camera_msgs::msg::Error,
}

impl Default for FeatureEnumInfoGetResponse {
    fn default() -> Self {
        FeatureEnumInfoGetResponse {
            possible_values: Vec::new(),
            available_values: Vec::new(),
            error: crate::vimbax_camera_msgs::msg::Error::default(),
        }
    }
}

impl ros2_client::Message for FeatureEnumInfoGetResponse {}


pub struct FeatureEnumInfoGet;
impl ros2_client::Service for FeatureEnumInfoGet {
    type Request = FeatureEnumInfoGetRequest;
    type Response = FeatureEnumInfoGetResponse;

    fn request_type_name(&self) -> &str { "FeatureEnumInfoGetRequest" }
    fn response_type_name(&self) -> &str { "FeatureEnumInfoGetResponse" }
}
