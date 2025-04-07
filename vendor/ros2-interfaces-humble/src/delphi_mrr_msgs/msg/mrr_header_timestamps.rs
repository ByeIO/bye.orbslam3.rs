use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MrrHeaderTimestamps {
    pub header: crate::std_msgs::msg::Header,
    pub can_det_time_since_meas: f32,
    pub can_sensor_time_stamp: f32,
}

impl Default for MrrHeaderTimestamps {
    fn default() -> Self {
        MrrHeaderTimestamps {
            header: crate::std_msgs::msg::Header::default(),
            can_det_time_since_meas: 0.0,
            can_sensor_time_stamp: 0.0,
        }
    }
}

impl ros2_client::Message for MrrHeaderTimestamps {}
