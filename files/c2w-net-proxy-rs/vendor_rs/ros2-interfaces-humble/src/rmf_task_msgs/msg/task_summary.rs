use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaskSummary {
    pub fleet_name: ::std::string::String,
    pub task_id: ::std::string::String,
    pub task_profile: crate::rmf_task_msgs::msg::TaskProfile,
    pub state: u32,
    pub status: ::std::string::String,
    pub submission_time: crate::builtin_interfaces::msg::Time,
    pub start_time: crate::builtin_interfaces::msg::Time,
    pub end_time: crate::builtin_interfaces::msg::Time,
    pub robot_name: ::std::string::String,
}

impl TaskSummary {
    pub const STATE_QUEUED: u32 = 0;
    pub const STATE_ACTIVE: u32 = 1;
    pub const STATE_COMPLETED: u32 = 2;
    pub const STATE_FAILED: u32 = 3;
    pub const STATE_CANCELED: u32 = 4;
    pub const STATE_PENDING: u32 = 5;
}

impl Default for TaskSummary {
    fn default() -> Self {
        TaskSummary {
            fleet_name: ::std::string::String::new(),
            task_id: ::std::string::String::new(),
            task_profile: crate::rmf_task_msgs::msg::TaskProfile::default(),
            state: 0,
            status: ::std::string::String::new(),
            submission_time: crate::builtin_interfaces::msg::Time::default(),
            start_time: crate::builtin_interfaces::msg::Time::default(),
            end_time: crate::builtin_interfaces::msg::Time::default(),
            robot_name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for TaskSummary {}
