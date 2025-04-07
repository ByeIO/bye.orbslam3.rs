use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeatureInfoQueryRequest {
    pub feature_names: Vec<::std::string::String>,
    pub feature_module: crate::vimbax_camera_msgs::msg::FeatureModule,
}

impl Default for FeatureInfoQueryRequest {
    fn default() -> Self {
        FeatureInfoQueryRequest {
            feature_names: Vec::new(),
            feature_module: crate::vimbax_camera_msgs::msg::FeatureModule::default(),
        }
    }
}

impl ros2_client::Message for FeatureInfoQueryRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeatureInfoQueryResponse {
    pub feature_info: Vec<crate::vimbax_camera_msgs::msg::FeatureInfo>,
    pub error: crate::vimbax_camera_msgs::msg::Error,
}

impl Default for FeatureInfoQueryResponse {
    fn default() -> Self {
        FeatureInfoQueryResponse {
            feature_info: Vec::new(),
            error: crate::vimbax_camera_msgs::msg::Error::default(),
        }
    }
}

impl ros2_client::Message for FeatureInfoQueryResponse {}


pub struct FeatureInfoQuery;
impl ros2_client::Service for FeatureInfoQuery {
    type Request = FeatureInfoQueryRequest;
    type Response = FeatureInfoQueryResponse;

    fn request_type_name(&self) -> &str { "FeatureInfoQueryRequest" }
    fn response_type_name(&self) -> &str { "FeatureInfoQueryResponse" }
}
