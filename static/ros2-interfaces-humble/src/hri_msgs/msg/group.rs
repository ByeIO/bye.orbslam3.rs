use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Group {
    pub header: crate::std_msgs::msg::Header,
    pub group_id: ::std::string::String,
    pub members: Vec<::std::string::String>,
}

impl Default for Group {
    fn default() -> Self {
        Group {
            header: crate::std_msgs::msg::Header::default(),
            group_id: ::std::string::String::new(),
            members: Vec::new(),
        }
    }
}

impl ros2_client::Message for Group {}
