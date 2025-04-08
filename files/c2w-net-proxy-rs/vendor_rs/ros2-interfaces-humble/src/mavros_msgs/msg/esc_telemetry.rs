use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ESCTelemetry {
    pub header: crate::std_msgs::msg::Header,
    pub esc_telemetry: Vec<crate::mavros_msgs::msg::ESCTelemetryItem>,
}

impl Default for ESCTelemetry {
    fn default() -> Self {
        ESCTelemetry {
            header: crate::std_msgs::msg::Header::default(),
            esc_telemetry: Vec::new(),
        }
    }
}

impl ros2_client::Message for ESCTelemetry {}
