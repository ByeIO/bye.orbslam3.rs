use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeatureStringSetRequest {
    pub feature_name: ::std::string::String,
    pub feature_module: crate::vimbax_camera_msgs::msg::FeatureModule,
    pub value: ::std::string::String,
}

impl Default for FeatureStringSetRequest {
    fn default() -> Self {
        FeatureStringSetRequest {
            feature_name: ::std::string::String::new(),
            feature_module: crate::vimbax_camera_msgs::msg::FeatureModule::default(),
            value: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for FeatureStringSetRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeatureStringSetResponse {
    pub error: crate::vimbax_camera_msgs::msg::Error,
}

impl Default for FeatureStringSetResponse {
    fn default() -> Self {
        FeatureStringSetResponse {
            error: crate::vimbax_camera_msgs::msg::Error::default(),
        }
    }
}

impl ros2_client::Message for FeatureStringSetResponse {}


pub struct FeatureStringSet;
impl ros2_client::Service for FeatureStringSet {
    type Request = FeatureStringSetRequest;
    type Response = FeatureStringSetResponse;

    fn request_type_name(&self) -> &str { "FeatureStringSetRequest" }
    fn response_type_name(&self) -> &str { "FeatureStringSetResponse" }
}
