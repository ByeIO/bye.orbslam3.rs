use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Station {
    pub header: crate::std_msgs::msg::Header,
    pub id: i32,
    pub name: ::std::string::String,
    pub pose: crate::geometry_msgs::msg::Pose,
}

impl Default for Station {
    fn default() -> Self {
        Station {
            header: crate::std_msgs::msg::Header::default(),
            id: 0,
            name: ::std::string::String::new(),
            pose: crate::geometry_msgs::msg::Pose::default(),
        }
    }
}

impl ros2_client::Message for Station {}
