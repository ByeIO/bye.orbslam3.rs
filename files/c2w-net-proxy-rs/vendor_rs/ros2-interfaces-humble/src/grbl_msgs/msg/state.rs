use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct State {
    pub header: crate::std_msgs::msg::Header,
    pub state_name: ::std::string::String,
    pub state: u8,
}

impl Default for State {
    fn default() -> Self {
        State {
            header: crate::std_msgs::msg::Header::default(),
            state_name: ::std::string::String::new(),
            state: 0,
        }
    }
}

impl ros2_client::Message for State {}
