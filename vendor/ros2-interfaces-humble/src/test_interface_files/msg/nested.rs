use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Nested {
    pub basic_types_value: crate::test_interface_files::msg::BasicTypes,
}

impl Default for Nested {
    fn default() -> Self {
        Nested {
            basic_types_value: crate::test_interface_files::msg::BasicTypes::default(),
        }
    }
}

impl ros2_client::Message for Nested {}
