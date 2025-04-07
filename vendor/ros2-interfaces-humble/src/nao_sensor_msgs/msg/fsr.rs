use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FSR {
    pub l_foot_front_left: f32,
    pub l_foot_front_right: f32,
    pub l_foot_back_left: f32,
    pub l_foot_back_right: f32,
    pub r_foot_front_left: f32,
    pub r_foot_front_right: f32,
    pub r_foot_back_left: f32,
    pub r_foot_back_right: f32,
}

impl Default for FSR {
    fn default() -> Self {
        FSR {
            l_foot_front_left: 0.0,
            l_foot_front_right: 0.0,
            l_foot_back_left: 0.0,
            l_foot_back_right: 0.0,
            r_foot_front_left: 0.0,
            r_foot_front_right: 0.0,
            r_foot_back_left: 0.0,
            r_foot_back_right: 0.0,
        }
    }
}

impl ros2_client::Message for FSR {}
