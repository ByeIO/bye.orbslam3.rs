use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClientGlobalAlignLandmarkObservationNotice {
    pub pose_index: u32,
    pub landmark_index: u32,
}

impl Default for ClientGlobalAlignLandmarkObservationNotice {
    fn default() -> Self {
        ClientGlobalAlignLandmarkObservationNotice {
            pose_index: 0,
            landmark_index: 0,
        }
    }
}

impl ros2_client::Message for ClientGlobalAlignLandmarkObservationNotice {}
