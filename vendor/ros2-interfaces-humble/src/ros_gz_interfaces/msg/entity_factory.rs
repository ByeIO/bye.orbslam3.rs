use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntityFactory {
    pub name: ::std::string::String,
    pub allow_renaming: bool, // default: false
    pub sdf: ::std::string::String,
    pub sdf_filename: ::std::string::String,
    pub clone_name: ::std::string::String,
    pub pose: crate::geometry_msgs::msg::Pose,
    pub relative_to: ::std::string::String, // default: "world"
}

impl Default for EntityFactory {
    fn default() -> Self {
        EntityFactory {
            name: ::std::string::String::new(),
            allow_renaming: false,
            sdf: ::std::string::String::new(),
            sdf_filename: ::std::string::String::new(),
            clone_name: ::std::string::String::new(),
            pose: crate::geometry_msgs::msg::Pose::default(),
            relative_to: ::std::string::String::from("world"),
        }
    }
}

impl ros2_client::Message for EntityFactory {}
