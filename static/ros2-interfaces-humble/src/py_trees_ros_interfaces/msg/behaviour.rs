use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Behaviour {
    pub name: ::std::string::String,
    pub class_name: ::std::string::String,
    pub own_id: crate::unique_identifier_msgs::msg::UUID,
    pub parent_id: crate::unique_identifier_msgs::msg::UUID,
    pub tip_id: crate::unique_identifier_msgs::msg::UUID,
    pub child_ids: Vec<crate::unique_identifier_msgs::msg::UUID>,
    pub current_child_id: crate::unique_identifier_msgs::msg::UUID,
    #[serde(rename = "type")]    pub type_: u8,
    pub additional_detail: ::std::string::String,
    pub blackbox_level: u8,
    pub status: u8,
    pub message: ::std::string::String,
    pub is_active: bool,
    pub blackboard_access: Vec<crate::py_trees_ros_interfaces::msg::KeyValue>,
}

impl Behaviour {
    pub const INVALID: u8 = 1;
    pub const RUNNING: u8 = 2;
    pub const SUCCESS: u8 = 3;
    pub const FAILURE: u8 = 4;
    pub const UNKNOWN_TYPE: u8 = 0;
    pub const BEHAVIOUR: u8 = 1;
    pub const SEQUENCE: u8 = 2;
    pub const SELECTOR: u8 = 3;
    pub const PARALLEL: u8 = 4;
    pub const CHOOSER: u8 = 5;
    pub const DECORATOR: u8 = 6;
    pub const BLACKBOX_LEVEL_DETAIL: u8 = 1;
    pub const BLACKBOX_LEVEL_COMPONENT: u8 = 2;
    pub const BLACKBOX_LEVEL_BIG_PICTURE: u8 = 3;
    pub const BLACKBOX_LEVEL_NOT_A_BLACKBOX: u8 = 4;
    pub const BLACKBOARD_ACCESS_READ: &'static str = "r";
    pub const BLACKBOARD_ACCESS_WRITE: &'static str = "w";
    pub const BLACKBOARD_ACCESS_EXCLUSIVE_WRITE: &'static str = "x";
}

impl Default for Behaviour {
    fn default() -> Self {
        Behaviour {
            name: ::std::string::String::new(),
            class_name: ::std::string::String::new(),
            own_id: crate::unique_identifier_msgs::msg::UUID::default(),
            parent_id: crate::unique_identifier_msgs::msg::UUID::default(),
            tip_id: crate::unique_identifier_msgs::msg::UUID::default(),
            child_ids: Vec::new(),
            current_child_id: crate::unique_identifier_msgs::msg::UUID::default(),
            type_: 0,
            additional_detail: ::std::string::String::new(),
            blackbox_level: 0,
            status: 0,
            message: ::std::string::String::new(),
            is_active: false,
            blackboard_access: Vec::new(),
        }
    }
}

impl ros2_client::Message for Behaviour {}
