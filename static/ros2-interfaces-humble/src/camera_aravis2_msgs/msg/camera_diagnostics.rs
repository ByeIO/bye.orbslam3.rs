use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraDiagnostics {
    pub header: crate::std_msgs::msg::Header,
    pub data: Vec<crate::diagnostic_msgs::msg::KeyValue>,
}

impl Default for CameraDiagnostics {
    fn default() -> Self {
        CameraDiagnostics {
            header: crate::std_msgs::msg::Header::default(),
            data: Vec::new(),
        }
    }
}

impl ros2_client::Message for CameraDiagnostics {}
