use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorldControl {
    pub pause: bool,
    pub step: bool,
    pub multi_step: u32, // default: 0
    pub reset: crate::ros_gz_interfaces::msg::WorldReset,
    pub seed: u32,
    pub run_to_sim_time: crate::builtin_interfaces::msg::Time,
}

impl Default for WorldControl {
    fn default() -> Self {
        WorldControl {
            pause: false,
            step: false,
            multi_step: 0,
            reset: crate::ros_gz_interfaces::msg::WorldReset::default(),
            seed: 0,
            run_to_sim_time: crate::builtin_interfaces::msg::Time::default(),
        }
    }
}

impl ros2_client::Message for WorldControl {}
