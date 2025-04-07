use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VisibilityConstraint {
    pub target_radius: f64,
    pub target_pose: crate::geometry_msgs::msg::PoseStamped,
    pub cone_sides: i32,
    pub sensor_pose: crate::geometry_msgs::msg::PoseStamped,
    pub max_view_angle: f64,
    pub max_range_angle: f64,
    pub sensor_view_direction: u8,
    pub weight: f64,
}

impl VisibilityConstraint {
    pub const SENSOR_Z: u8 = 0;
    pub const SENSOR_Y: u8 = 1;
    pub const SENSOR_X: u8 = 2;
}

impl Default for VisibilityConstraint {
    fn default() -> Self {
        VisibilityConstraint {
            target_radius: 0.0,
            target_pose: crate::geometry_msgs::msg::PoseStamped::default(),
            cone_sides: 0,
            sensor_pose: crate::geometry_msgs::msg::PoseStamped::default(),
            max_view_angle: 0.0,
            max_range_angle: 0.0,
            sensor_view_direction: 0,
            weight: 0.0,
        }
    }
}

impl ros2_client::Message for VisibilityConstraint {}
