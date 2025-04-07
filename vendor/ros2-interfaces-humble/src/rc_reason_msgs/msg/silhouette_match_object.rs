use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SilhouetteMatchObject {
    pub object_id: ::std::string::String,
    pub region_of_interest_2d_id: ::std::string::String,
}

impl Default for SilhouetteMatchObject {
    fn default() -> Self {
        SilhouetteMatchObject {
            object_id: ::std::string::String::new(),
            region_of_interest_2d_id: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SilhouetteMatchObject {}
