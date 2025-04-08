use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConnectedClients {
    pub clients: Vec<crate::rosbridge_msgs::msg::ConnectedClient>,
}

impl Default for ConnectedClients {
    fn default() -> Self {
        ConnectedClients {
            clients: Vec::new(),
        }
    }
}

impl ros2_client::Message for ConnectedClients {}
