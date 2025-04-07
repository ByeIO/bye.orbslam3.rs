use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Constants {

}

impl Constants {
    pub const DEAD_PUBLISH_PERIOD: f32 = 0.05;
    pub const DEFAULT_CONNECT_TIMEOUT: f32 = 10.0;
    pub const DEFAULT_HEARTBEAT_TIMEOUT: f32 = 4.0;
    pub const DEFAULT_DISCONNECT_TIMEOUT: f32 = 2.0;
    pub const DEFAULT_HEARTBEAT_PERIOD: f32 = 1.0;
    pub const DISABLE_HEARTBEAT_TIMEOUT_PARAM: &'static str = "/bond_disable_heartbeat_timeout";
}

impl Default for Constants {
    fn default() -> Self {
        Constants {

        }
    }
}

impl ros2_client::Message for Constants {}
