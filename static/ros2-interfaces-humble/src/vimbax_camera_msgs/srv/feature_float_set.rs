use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeatureFloatSetRequest {
    pub feature_name: ::std::string::String,
    pub feature_module: crate::vimbax_camera_msgs::msg::FeatureModule,
    pub value: f64,
}

impl Default for FeatureFloatSetRequest {
    fn default() -> Self {
        FeatureFloatSetRequest {
            feature_name: ::std::string::String::new(),
            feature_module: crate::vimbax_camera_msgs::msg::FeatureModule::default(),
            value: 0.0,
        }
    }
}

impl ros2_client::Message for FeatureFloatSetRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeatureFloatSetResponse {
    pub error: crate::vimbax_camera_msgs::msg::Error,
}

impl Default for FeatureFloatSetResponse {
    fn default() -> Self {
        FeatureFloatSetResponse {
            error: crate::vimbax_camera_msgs::msg::Error::default(),
        }
    }
}

impl ros2_client::Message for FeatureFloatSetResponse {}


pub struct FeatureFloatSet;
impl ros2_client::Service for FeatureFloatSet {
    type Request = FeatureFloatSetRequest;
    type Response = FeatureFloatSetResponse;

    fn request_type_name(&self) -> &str { "FeatureFloatSetRequest" }
    fn response_type_name(&self) -> &str { "FeatureFloatSetResponse" }
}
