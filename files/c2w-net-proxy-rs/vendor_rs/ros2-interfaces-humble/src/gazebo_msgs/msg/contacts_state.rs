use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContactsState {
    pub header: crate::std_msgs::msg::Header,
    pub states: Vec<crate::gazebo_msgs::msg::ContactState>,
}

impl Default for ContactsState {
    fn default() -> Self {
        ContactsState {
            header: crate::std_msgs::msg::Header::default(),
            states: Vec::new(),
        }
    }
}

impl ros2_client::Message for ContactsState {}
