use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClientGlobalAlignLandmarkVisualizationInformation {
    #[serde(rename = "type")]    pub type_: i64,
    pub has_orientation: bool,
    pub name: ::std::string::String,
}

impl Default for ClientGlobalAlignLandmarkVisualizationInformation {
    fn default() -> Self {
        ClientGlobalAlignLandmarkVisualizationInformation {
            type_: 0,
            has_orientation: false,
            name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ClientGlobalAlignLandmarkVisualizationInformation {}
