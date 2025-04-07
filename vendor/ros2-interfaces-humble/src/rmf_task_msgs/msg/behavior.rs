use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Behavior {
    pub name: ::std::string::String,
    pub parameters: Vec<crate::rmf_task_msgs::msg::BehaviorParameter>,
}

impl Default for Behavior {
    fn default() -> Self {
        Behavior {
            name: ::std::string::String::new(),
            parameters: Vec::new(),
        }
    }
}

impl ros2_client::Message for Behavior {}
