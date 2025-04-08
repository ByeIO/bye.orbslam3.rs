use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeatureIntSetRequest {
    pub feature_name: ::std::string::String,
    pub feature_module: crate::vimbax_camera_msgs::msg::FeatureModule,
    pub value: i64,
}

impl Default for FeatureIntSetRequest {
    fn default() -> Self {
        FeatureIntSetRequest {
            feature_name: ::std::string::String::new(),
            feature_module: crate::vimbax_camera_msgs::msg::FeatureModule::default(),
            value: 0,
        }
    }
}

impl ros2_client::Message for FeatureIntSetRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeatureIntSetResponse {
    pub error: crate::vimbax_camera_msgs::msg::Error,
}

impl Default for FeatureIntSetResponse {
    fn default() -> Self {
        FeatureIntSetResponse {
            error: crate::vimbax_camera_msgs::msg::Error::default(),
        }
    }
}

impl ros2_client::Message for FeatureIntSetResponse {}


pub struct FeatureIntSet;
impl ros2_client::Service for FeatureIntSet {
    type Request = FeatureIntSetRequest;
    type Response = FeatureIntSetResponse;

    fn request_type_name(&self) -> &str { "FeatureIntSetRequest" }
    fn response_type_name(&self) -> &str { "FeatureIntSetResponse" }
}
