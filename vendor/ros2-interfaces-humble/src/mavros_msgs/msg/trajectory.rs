use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Trajectory {
    pub header: crate::std_msgs::msg::Header,
    #[serde(rename = "type")]    pub type_: u8,
    pub point_1: crate::mavros_msgs::msg::PositionTarget,
    pub point_2: crate::mavros_msgs::msg::PositionTarget,
    pub point_3: crate::mavros_msgs::msg::PositionTarget,
    pub point_4: crate::mavros_msgs::msg::PositionTarget,
    pub point_5: crate::mavros_msgs::msg::PositionTarget,
    pub point_valid: [u8; 5],
    pub command: [u16; 5],
    pub time_horizon: [f32; 5],
}

impl Trajectory {
    pub const MAV_TRAJECTORY_REPRESENTATION_WAYPOINTS: u8 = 0;
    pub const MAV_TRAJECTORY_REPRESENTATION_BEZIER: u8 = 1;
}

impl Default for Trajectory {
    fn default() -> Self {
        Trajectory {
            header: crate::std_msgs::msg::Header::default(),
            type_: 0,
            point_1: crate::mavros_msgs::msg::PositionTarget::default(),
            point_2: crate::mavros_msgs::msg::PositionTarget::default(),
            point_3: crate::mavros_msgs::msg::PositionTarget::default(),
            point_4: crate::mavros_msgs::msg::PositionTarget::default(),
            point_5: crate::mavros_msgs::msg::PositionTarget::default(),
            point_valid: [0; 5],
            command: [0; 5],
            time_horizon: [0.0; 5],
        }
    }
}

impl ros2_client::Message for Trajectory {}
