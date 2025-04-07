use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GlobalBundleAdjustmentRequest {
    #[serde(rename = "type")]    pub type_: i32,
    pub iterations: i32,
    pub pixel_variance: f32,
    pub voc_matches: bool,
}

impl Default for GlobalBundleAdjustmentRequest {
    fn default() -> Self {
        GlobalBundleAdjustmentRequest {
            type_: 0,
            iterations: 0,
            pixel_variance: 0.0,
            voc_matches: false,
        }
    }
}

impl ros2_client::Message for GlobalBundleAdjustmentRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GlobalBundleAdjustmentResponse {

}

impl Default for GlobalBundleAdjustmentResponse {
    fn default() -> Self {
        GlobalBundleAdjustmentResponse {

        }
    }
}

impl ros2_client::Message for GlobalBundleAdjustmentResponse {}


pub struct GlobalBundleAdjustment;
impl ros2_client::Service for GlobalBundleAdjustment {
    type Request = GlobalBundleAdjustmentRequest;
    type Response = GlobalBundleAdjustmentResponse;

    fn request_type_name(&self) -> &str { "GlobalBundleAdjustmentRequest" }
    fn response_type_name(&self) -> &str { "GlobalBundleAdjustmentResponse" }
}
