use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NestedTypes {
    pub data_basic_types: crate::r2r_spl_test_interfaces::msg::BasicTypes,
    pub data_basic_types_array: Vec<crate::r2r_spl_test_interfaces::msg::BasicTypes>,
}

impl Default for NestedTypes {
    fn default() -> Self {
        NestedTypes {
            data_basic_types: crate::r2r_spl_test_interfaces::msg::BasicTypes::default(),
            data_basic_types_array: Vec::new(),
        }
    }
}

impl ros2_client::Message for NestedTypes {}
