use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DetectMoreLoopClosuresRequest {
    pub cluster_radius_max: f32,
    pub cluster_radius_min: f32,
    pub cluster_angle: f32,
    pub iterations: i32,
    pub intra_only: bool,
    pub inter_only: bool,
}

impl Default for DetectMoreLoopClosuresRequest {
    fn default() -> Self {
        DetectMoreLoopClosuresRequest {
            cluster_radius_max: 0.0,
            cluster_radius_min: 0.0,
            cluster_angle: 0.0,
            iterations: 0,
            intra_only: false,
            inter_only: false,
        }
    }
}

impl ros2_client::Message for DetectMoreLoopClosuresRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DetectMoreLoopClosuresResponse {
    pub detected: i32,
}

impl Default for DetectMoreLoopClosuresResponse {
    fn default() -> Self {
        DetectMoreLoopClosuresResponse {
            detected: 0,
        }
    }
}

impl ros2_client::Message for DetectMoreLoopClosuresResponse {}


pub struct DetectMoreLoopClosures;
impl ros2_client::Service for DetectMoreLoopClosures {
    type Request = DetectMoreLoopClosuresRequest;
    type Response = DetectMoreLoopClosuresResponse;

    fn request_type_name(&self) -> &str { "DetectMoreLoopClosuresRequest" }
    fn response_type_name(&self) -> &str { "DetectMoreLoopClosuresResponse" }
}
