use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserButton {
    pub button: [bool; 4],
}

impl Default for UserButton {
    fn default() -> Self {
        UserButton {
            button: [false; 4],
        }
    }
}

impl ros2_client::Message for UserButton {}
