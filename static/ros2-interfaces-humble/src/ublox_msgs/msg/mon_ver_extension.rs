use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MonVERExtension {
    pub field: [i8; 30],
}

impl Default for MonVERExtension {
    fn default() -> Self {
        MonVERExtension {
            field: [0; 30],
        }
    }
}

impl ros2_client::Message for MonVERExtension {}
