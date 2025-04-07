use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClientGlobalAlignVisualization {
    pub timestamp: crate::builtin_interfaces::msg::Time,
    pub visualization_id: u64,
    pub landmarks: Vec<crate::bosch_locator_bridge::msg::ClientGlobalAlignLandmarkVisualizationInformation>,
    pub observations: Vec<crate::bosch_locator_bridge::msg::ClientGlobalAlignLandmarkObservationNotice>,
}

impl Default for ClientGlobalAlignVisualization {
    fn default() -> Self {
        ClientGlobalAlignVisualization {
            timestamp: crate::builtin_interfaces::msg::Time::default(),
            visualization_id: 0,
            landmarks: Vec::new(),
            observations: Vec::new(),
        }
    }
}

impl ros2_client::Message for ClientGlobalAlignVisualization {}
