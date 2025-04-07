use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GnssInsOrientation {
    pub orientation: crate::geometry_msgs::msg::Quaternion,
    pub rmse_rotation_x: f32,
    pub rmse_rotation_y: f32,
    pub rmse_rotation_z: f32,
}

impl Default for GnssInsOrientation {
    fn default() -> Self {
        GnssInsOrientation {
            orientation: crate::geometry_msgs::msg::Quaternion::default(),
            rmse_rotation_x: 0.0,
            rmse_rotation_y: 0.0,
            rmse_rotation_z: 0.0,
        }
    }
}

impl ros2_client::Message for GnssInsOrientation {}
