use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RttestResults {
    pub stamp: crate::builtin_interfaces::msg::Time,
    pub command: crate::pendulum_msgs::msg::JointCommand,
    pub state: crate::pendulum_msgs::msg::JointState,
    pub cur_latency: u64,
    pub mean_latency: f64,
    pub min_latency: u64,
    pub max_latency: u64,
    pub minor_pagefaults: u64,
    pub major_pagefaults: u64,
}

impl Default for RttestResults {
    fn default() -> Self {
        RttestResults {
            stamp: crate::builtin_interfaces::msg::Time::default(),
            command: crate::pendulum_msgs::msg::JointCommand::default(),
            state: crate::pendulum_msgs::msg::JointState::default(),
            cur_latency: 0,
            mean_latency: 0.0,
            min_latency: 0,
            max_latency: 0,
            minor_pagefaults: 0,
            major_pagefaults: 0,
        }
    }
}

impl ros2_client::Message for RttestResults {}
