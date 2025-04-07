use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Heartbeat {

}

impl Default for Heartbeat {
    fn default() -> Self {
        Heartbeat {

        }
    }
}

impl ros2_client::Message for Heartbeat {}
