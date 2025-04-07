use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrientationConstraint {
    pub header: crate::std_msgs::msg::Header,
    pub orientation: crate::geometry_msgs::msg::Quaternion,
    pub link_name: ::std::string::String,
    pub absolute_x_axis_tolerance: f64,
    pub absolute_y_axis_tolerance: f64,
    pub absolute_z_axis_tolerance: f64,
    pub parameterization: u8,
    pub weight: f64,
}

impl OrientationConstraint {
    pub const XYZ_EULER_ANGLES: u8 = 0;
    pub const ROTATION_VECTOR: u8 = 1;
}

impl Default for OrientationConstraint {
    fn default() -> Self {
        OrientationConstraint {
            header: crate::std_msgs::msg::Header::default(),
            orientation: crate::geometry_msgs::msg::Quaternion::default(),
            link_name: ::std::string::String::new(),
            absolute_x_axis_tolerance: 0.0,
            absolute_y_axis_tolerance: 0.0,
            absolute_z_axis_tolerance: 0.0,
            parameterization: 0,
            weight: 0.0,
        }
    }
}

impl ros2_client::Message for OrientationConstraint {}
