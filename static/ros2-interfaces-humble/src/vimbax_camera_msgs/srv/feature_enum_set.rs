use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeatureEnumSetRequest {
    pub feature_name: ::std::string::String,
    pub feature_module: crate::vimbax_camera_msgs::msg::FeatureModule,
    pub value: ::std::string::String,
}

impl Default for FeatureEnumSetRequest {
    fn default() -> Self {
        FeatureEnumSetRequest {
            feature_name: ::std::string::String::new(),
            feature_module: crate::vimbax_camera_msgs::msg::FeatureModule::default(),
            value: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for FeatureEnumSetRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeatureEnumSetResponse {
    pub error: crate::vimbax_camera_msgs::msg::Error,
}

impl Default for FeatureEnumSetResponse {
    fn default() -> Self {
        FeatureEnumSetResponse {
            error: crate::vimbax_camera_msgs::msg::Error::default(),
        }
    }
}

impl ros2_client::Message for FeatureEnumSetResponse {}


pub struct FeatureEnumSet;
impl ros2_client::Service for FeatureEnumSet {
    type Request = FeatureEnumSetRequest;
    type Response = FeatureEnumSetResponse;

    fn request_type_name(&self) -> &str { "FeatureEnumSetRequest" }
    fn response_type_name(&self) -> &str { "FeatureEnumSetResponse" }
}
