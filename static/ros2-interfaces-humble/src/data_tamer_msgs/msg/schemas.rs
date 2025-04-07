use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Schemas {
    pub schemas: Vec<crate::data_tamer_msgs::msg::Schema>,
}

impl Default for Schemas {
    fn default() -> Self {
        Schemas {
            schemas: Vec::new(),
        }
    }
}

impl ros2_client::Message for Schemas {}
