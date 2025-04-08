use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct State {
    pub actuators: Vec<crate::qb_device_msgs::msg::ResourceData>,
    pub joints: Vec<crate::qb_device_msgs::msg::ResourceData>,
    pub is_reliable: bool,
    pub consecutive_failures: i32,
}

impl Default for State {
    fn default() -> Self {
        State {
            actuators: Vec::new(),
            joints: Vec::new(),
            is_reliable: false,
            consecutive_failures: 0,
        }
    }
}

impl ros2_client::Message for State {}
