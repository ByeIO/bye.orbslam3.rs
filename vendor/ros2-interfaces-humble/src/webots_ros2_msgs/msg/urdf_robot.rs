use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UrdfRobot {
    pub name: ::std::string::String,
    pub urdf_path: ::std::string::String,
    pub robot_description: ::std::string::String,
    pub relative_path_prefix: ::std::string::String,
    pub translation: ::std::string::String,
    pub rotation: ::std::string::String,
    pub normal: bool,
    pub box_collision: bool,
    pub init_pos: ::std::string::String,
}

impl Default for UrdfRobot {
    fn default() -> Self {
        UrdfRobot {
            name: ::std::string::String::new(),
            urdf_path: ::std::string::String::new(),
            robot_description: ::std::string::String::new(),
            relative_path_prefix: ::std::string::String::new(),
            translation: ::std::string::String::new(),
            rotation: ::std::string::String::new(),
            normal: false,
            box_collision: false,
            init_pos: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for UrdfRobot {}
