use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tow {
    pub task_id: ::std::string::String,
    pub object_type: ::std::string::String,
    pub is_object_id_known: bool,
    pub object_id: ::std::string::String,
    pub pickup_place_name: ::std::string::String,
    pub is_dropoff_place_known: bool,
    pub dropoff_place_name: ::std::string::String,
}

impl Default for Tow {
    fn default() -> Self {
        Tow {
            task_id: ::std::string::String::new(),
            object_type: ::std::string::String::new(),
            is_object_id_known: false,
            object_id: ::std::string::String::new(),
            pickup_place_name: ::std::string::String::new(),
            is_dropoff_place_known: false,
            dropoff_place_name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for Tow {}
