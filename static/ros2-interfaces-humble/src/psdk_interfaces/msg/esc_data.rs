use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EscData {
    pub header: crate::std_msgs::msg::Header,
    pub esc: Vec<crate::psdk_interfaces::msg::EscStatusIndividual>,
}

impl Default for EscData {
    fn default() -> Self {
        EscData {
            header: crate::std_msgs::msg::Header::default(),
            esc: Vec::new(),
        }
    }
}

impl ros2_client::Message for EscData {}
