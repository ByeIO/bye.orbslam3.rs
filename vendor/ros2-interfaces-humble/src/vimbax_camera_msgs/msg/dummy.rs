use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Dummy {

}

impl Default for Dummy {
    fn default() -> Self {
        Dummy {

        }
    }
}

impl ros2_client::Message for Dummy {}
