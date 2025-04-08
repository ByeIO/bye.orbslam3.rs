use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct End {

}

impl Default for End {
    fn default() -> Self {
        End {

        }
    }
}

impl ros2_client::Message for End {}
