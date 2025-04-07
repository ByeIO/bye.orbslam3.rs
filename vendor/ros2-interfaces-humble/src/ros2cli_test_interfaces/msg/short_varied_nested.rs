use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShortVariedNested {
    pub short_varied: crate::ros2cli_test_interfaces::msg::ShortVaried,
}

impl Default for ShortVariedNested {
    fn default() -> Self {
        ShortVariedNested {
            short_varied: crate::ros2cli_test_interfaces::msg::ShortVaried::default(),
        }
    }
}

impl ros2_client::Message for ShortVariedNested {}
