use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Inertia {
    pub m: f64,
    pub com: crate::geometry_msgs::msg::Vector3,
    pub ixx: f64,
    pub ixy: f64,
    pub ixz: f64,
    pub iyy: f64,
    pub iyz: f64,
    pub izz: f64,
}

impl Default for Inertia {
    fn default() -> Self {
        Inertia {
            m: 0.0,
            com: crate::geometry_msgs::msg::Vector3::default(),
            ixx: 0.0,
            ixy: 0.0,
            ixz: 0.0,
            iyy: 0.0,
            iyz: 0.0,
            izz: 0.0,
        }
    }
}

impl ros2_client::Message for Inertia {}
