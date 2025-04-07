use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeatureBoolSetRequest {
    pub feature_name: ::std::string::String,
    pub feature_module: crate::vimbax_camera_msgs::msg::FeatureModule,
    pub value: bool,
}

impl Default for FeatureBoolSetRequest {
    fn default() -> Self {
        FeatureBoolSetRequest {
            feature_name: ::std::string::String::new(),
            feature_module: crate::vimbax_camera_msgs::msg::FeatureModule::default(),
            value: false,
        }
    }
}

impl ros2_client::Message for FeatureBoolSetRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeatureBoolSetResponse {
    pub error: crate::vimbax_camera_msgs::msg::Error,
}

impl Default for FeatureBoolSetResponse {
    fn default() -> Self {
        FeatureBoolSetResponse {
            error: crate::vimbax_camera_msgs::msg::Error::default(),
        }
    }
}

impl ros2_client::Message for FeatureBoolSetResponse {}


pub struct FeatureBoolSet;
impl ros2_client::Service for FeatureBoolSet {
    type Request = FeatureBoolSetRequest;
    type Response = FeatureBoolSetResponse;

    fn request_type_name(&self) -> &str { "FeatureBoolSetRequest" }
    fn response_type_name(&self) -> &str { "FeatureBoolSetResponse" }
}
