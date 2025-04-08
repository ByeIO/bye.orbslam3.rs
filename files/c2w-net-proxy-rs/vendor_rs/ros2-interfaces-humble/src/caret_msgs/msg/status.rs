use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Status {
    pub caret_node_name: ::std::string::String,
    pub status: i8,
    pub node_names: Vec<::std::string::String>,
    pub pid: i64,
}

impl Status {
    pub const UNINITIALIZED: i8 = 0;
    pub const WAIT: i8 = 1;
    pub const PREPARE: i8 = 2;
    pub const RECORD: i8 = 3;
}

impl Default for Status {
    fn default() -> Self {
        Status {
            caret_node_name: ::std::string::String::new(),
            status: 0,
            node_names: Vec::new(),
            pid: 0,
        }
    }
}

impl ros2_client::Message for Status {}
