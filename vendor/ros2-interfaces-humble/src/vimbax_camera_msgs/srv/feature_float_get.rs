use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeatureFloatGetRequest {
    pub feature_name: ::std::string::String,
    pub feature_module: crate::vimbax_camera_msgs::msg::FeatureModule,
}

impl Default for FeatureFloatGetRequest {
    fn default() -> Self {
        FeatureFloatGetRequest {
            feature_name: ::std::string::String::new(),
            feature_module: crate::vimbax_camera_msgs::msg::FeatureModule::default(),
        }
    }
}

impl ros2_client::Message for FeatureFloatGetRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeatureFloatGetResponse {
    pub value: f64,
    pub error: crate::vimbax_camera_msgs::msg::Error,
}

impl Default for FeatureFloatGetResponse {
    fn default() -> Self {
        FeatureFloatGetResponse {
            value: 0.0,
            error: crate::vimbax_camera_msgs::msg::Error::default(),
        }
    }
}

impl ros2_client::Message for FeatureFloatGetResponse {}


pub struct FeatureFloatGet;
impl ros2_client::Service for FeatureFloatGet {
    type Request = FeatureFloatGetRequest;
    type Response = FeatureFloatGetResponse;

    fn request_type_name(&self) -> &str { "FeatureFloatGetRequest" }
    fn response_type_name(&self) -> &str { "FeatureFloatGetResponse" }
}
