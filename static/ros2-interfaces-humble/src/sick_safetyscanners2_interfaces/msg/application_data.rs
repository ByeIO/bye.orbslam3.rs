use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApplicationData {
    pub inputs: crate::sick_safetyscanners2_interfaces::msg::ApplicationInputs,
    pub outputs: crate::sick_safetyscanners2_interfaces::msg::ApplicationOutputs,
}

impl Default for ApplicationData {
    fn default() -> Self {
        ApplicationData {
            inputs: crate::sick_safetyscanners2_interfaces::msg::ApplicationInputs::default(),
            outputs: crate::sick_safetyscanners2_interfaces::msg::ApplicationOutputs::default(),
        }
    }
}

impl ros2_client::Message for ApplicationData {}
