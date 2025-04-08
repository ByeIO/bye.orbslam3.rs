use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Loop {
    pub task_id: ::std::string::String,
    pub robot_type: ::std::string::String,
    pub num_loops: u32,
    pub start_name: ::std::string::String,
    pub finish_name: ::std::string::String,
}

impl Default for Loop {
    fn default() -> Self {
        Loop {
            task_id: ::std::string::String::new(),
            robot_type: ::std::string::String::new(),
            num_loops: 0,
            start_name: ::std::string::String::new(),
            finish_name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for Loop {}
