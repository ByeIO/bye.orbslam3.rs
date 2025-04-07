use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ControllerState {
    pub header: crate::std_msgs::msg::Header,
    pub state: i32,
    pub progress: u32,
    pub progress_in_relation_to: u32,
    pub info: ::std::string::String,
}

impl ControllerState {
    pub const STATE_IDLE: i32 = 0;
    pub const STATE_DRIVING: i32 = 1;
    pub const STATE_FINISHED: i32 = 2;
}

impl Default for ControllerState {
    fn default() -> Self {
        ControllerState {
            header: crate::std_msgs::msg::Header::default(),
            state: 0,
            progress: 0,
            progress_in_relation_to: 0,
            info: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ControllerState {}
