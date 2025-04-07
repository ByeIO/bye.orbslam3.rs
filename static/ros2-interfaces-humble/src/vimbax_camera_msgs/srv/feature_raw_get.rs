use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeatureRawGetRequest {
    pub feature_name: ::std::string::String,
    pub feature_module: crate::vimbax_camera_msgs::msg::FeatureModule,
}

impl Default for FeatureRawGetRequest {
    fn default() -> Self {
        FeatureRawGetRequest {
            feature_name: ::std::string::String::new(),
            feature_module: crate::vimbax_camera_msgs::msg::FeatureModule::default(),
        }
    }
}

impl ros2_client::Message for FeatureRawGetRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeatureRawGetResponse {
    pub buffer: Vec<u8>,
    pub buffer_size: u32,
    pub error: crate::vimbax_camera_msgs::msg::Error,
}

impl Default for FeatureRawGetResponse {
    fn default() -> Self {
        FeatureRawGetResponse {
            buffer: Vec::new(),
            buffer_size: 0,
            error: crate::vimbax_camera_msgs::msg::Error::default(),
        }
    }
}

impl ros2_client::Message for FeatureRawGetResponse {}


pub struct FeatureRawGet;
impl ros2_client::Service for FeatureRawGet {
    type Request = FeatureRawGetRequest;
    type Response = FeatureRawGetResponse;

    fn request_type_name(&self) -> &str { "FeatureRawGetRequest" }
    fn response_type_name(&self) -> &str { "FeatureRawGetResponse" }
}
