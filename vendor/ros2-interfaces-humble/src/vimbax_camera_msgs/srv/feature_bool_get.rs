use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeatureBoolGetRequest {
    pub feature_name: ::std::string::String,
    pub feature_module: crate::vimbax_camera_msgs::msg::FeatureModule,
}

impl Default for FeatureBoolGetRequest {
    fn default() -> Self {
        FeatureBoolGetRequest {
            feature_name: ::std::string::String::new(),
            feature_module: crate::vimbax_camera_msgs::msg::FeatureModule::default(),
        }
    }
}

impl ros2_client::Message for FeatureBoolGetRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeatureBoolGetResponse {
    pub value: bool,
    pub error: crate::vimbax_camera_msgs::msg::Error,
}

impl Default for FeatureBoolGetResponse {
    fn default() -> Self {
        FeatureBoolGetResponse {
            value: false,
            error: crate::vimbax_camera_msgs::msg::Error::default(),
        }
    }
}

impl ros2_client::Message for FeatureBoolGetResponse {}


pub struct FeatureBoolGet;
impl ros2_client::Service for FeatureBoolGet {
    type Request = FeatureBoolGetRequest;
    type Response = FeatureBoolGetResponse;

    fn request_type_name(&self) -> &str { "FeatureBoolGetRequest" }
    fn response_type_name(&self) -> &str { "FeatureBoolGetResponse" }
}
