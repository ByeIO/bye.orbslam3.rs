use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModuleState {
    pub header: crate::std_msgs::msg::Header,
    pub name: ::std::string::String,
    pub state: ::std::string::String,
    pub info: ::std::string::String,
}

impl Default for ModuleState {
    fn default() -> Self {
        ModuleState {
            header: crate::std_msgs::msg::Header::default(),
            name: ::std::string::String::new(),
            state: ::std::string::String::new(),
            info: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ModuleState {}
