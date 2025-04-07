use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeatureRawSetRequest {
    pub feature_name: ::std::string::String,
    pub feature_module: crate::vimbax_camera_msgs::msg::FeatureModule,
    pub buffer: Vec<u8>,
}

impl Default for FeatureRawSetRequest {
    fn default() -> Self {
        FeatureRawSetRequest {
            feature_name: ::std::string::String::new(),
            feature_module: crate::vimbax_camera_msgs::msg::FeatureModule::default(),
            buffer: Vec::new(),
        }
    }
}

impl ros2_client::Message for FeatureRawSetRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeatureRawSetResponse {
    pub error: crate::vimbax_camera_msgs::msg::Error,
}

impl Default for FeatureRawSetResponse {
    fn default() -> Self {
        FeatureRawSetResponse {
            error: crate::vimbax_camera_msgs::msg::Error::default(),
        }
    }
}

impl ros2_client::Message for FeatureRawSetResponse {}


pub struct FeatureRawSet;
impl ros2_client::Service for FeatureRawSet {
    type Request = FeatureRawSetRequest;
    type Response = FeatureRawSetResponse;

    fn request_type_name(&self) -> &str { "FeatureRawSetRequest" }
    fn response_type_name(&self) -> &str { "FeatureRawSetResponse" }
}
