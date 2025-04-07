use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VfrHud {
    pub header: crate::std_msgs::msg::Header,
    pub airspeed: f32,
    pub groundspeed: f32,
    pub heading: i16,
    pub throttle: f32,
    pub altitude: f32,
    pub climb: f32,
}

impl Default for VfrHud {
    fn default() -> Self {
        VfrHud {
            header: crate::std_msgs::msg::Header::default(),
            airspeed: 0.0,
            groundspeed: 0.0,
            heading: 0,
            throttle: 0.0,
            altitude: 0.0,
            climb: 0.0,
        }
    }
}

impl ros2_client::Message for VfrHud {}
