use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeatureIntGetRequest {
    pub feature_name: ::std::string::String,
    pub feature_module: crate::vimbax_camera_msgs::msg::FeatureModule,
}

impl Default for FeatureIntGetRequest {
    fn default() -> Self {
        FeatureIntGetRequest {
            feature_name: ::std::string::String::new(),
            feature_module: crate::vimbax_camera_msgs::msg::FeatureModule::default(),
        }
    }
}

impl ros2_client::Message for FeatureIntGetRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeatureIntGetResponse {
    pub value: i64,
    pub error: crate::vimbax_camera_msgs::msg::Error,
}

impl Default for FeatureIntGetResponse {
    fn default() -> Self {
        FeatureIntGetResponse {
            value: 0,
            error: crate::vimbax_camera_msgs::msg::Error::default(),
        }
    }
}

impl ros2_client::Message for FeatureIntGetResponse {}


pub struct FeatureIntGet;
impl ros2_client::Service for FeatureIntGet {
    type Request = FeatureIntGetRequest;
    type Response = FeatureIntGetResponse;

    fn request_type_name(&self) -> &str { "FeatureIntGetRequest" }
    fn response_type_name(&self) -> &str { "FeatureIntGetResponse" }
}
