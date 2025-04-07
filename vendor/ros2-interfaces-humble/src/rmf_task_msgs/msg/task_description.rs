use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaskDescription {
    pub start_time: crate::builtin_interfaces::msg::Time,
    pub priority: crate::rmf_task_msgs::msg::Priority,
    pub task_type: crate::rmf_task_msgs::msg::TaskType,
    pub station: crate::rmf_task_msgs::msg::Station,
    #[serde(rename = "loop")]    pub loop_: crate::rmf_task_msgs::msg::Loop,
    pub delivery: crate::rmf_task_msgs::msg::Delivery,
    pub clean: crate::rmf_task_msgs::msg::Clean,
}

impl Default for TaskDescription {
    fn default() -> Self {
        TaskDescription {
            start_time: crate::builtin_interfaces::msg::Time::default(),
            priority: crate::rmf_task_msgs::msg::Priority::default(),
            task_type: crate::rmf_task_msgs::msg::TaskType::default(),
            station: crate::rmf_task_msgs::msg::Station::default(),
            loop_: crate::rmf_task_msgs::msg::Loop::default(),
            delivery: crate::rmf_task_msgs::msg::Delivery::default(),
            clean: crate::rmf_task_msgs::msg::Clean::default(),
        }
    }
}

impl ros2_client::Message for TaskDescription {}
