use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RobotInfo {
    pub header: crate::std_msgs::msg::Header,
    pub robot_name: ::std::string::String,
    pub pose: crate::geometry_msgs::msg::PoseWithCovariance,
    pub shape: i32,
    pub shape_variables: Vec<f32>,
    pub sync: crate::tuw_multi_robot_msgs::msg::RoutePrecondition,
    pub mode: i32,
    pub status: i32,
    pub good_id: i32,
    pub order_id: i32,
    pub order_status: i32,
}

impl RobotInfo {
    pub const MODE_NA: i32 = 0;
    pub const MODE_IDLE: i32 = 1;
    pub const MODE_SEGMENT_FOLLOWING: i32 = 2;
    pub const MODE_PICKUP: i32 = 3;
    pub const STATUS_DRIVING: i32 = 0;
    pub const STATUS_STOPPED: i32 = 1;
    pub const STATUS_DONE: i32 = 2;
    pub const STATUS_BROKEN: i32 = 3;
    pub const GOOD_EMPTY: i32 = -1;
    pub const GOOD_NA: i32 = -2;
    pub const SHAPE_CIRCLE: i32 = 0;
    pub const ORDER_NONE: i32 = 0;
    pub const ORDER_APPROACH: i32 = 1;
    pub const ORDER_PICKUP: i32 = 2;
    pub const ORDER_TRANSPORT: i32 = 3;
    pub const ORDER_DROP: i32 = 4;
}

impl Default for RobotInfo {
    fn default() -> Self {
        RobotInfo {
            header: crate::std_msgs::msg::Header::default(),
            robot_name: ::std::string::String::new(),
            pose: crate::geometry_msgs::msg::PoseWithCovariance::default(),
            shape: 0,
            shape_variables: Vec::new(),
            sync: crate::tuw_multi_robot_msgs::msg::RoutePrecondition::default(),
            mode: 0,
            status: 0,
            good_id: 0,
            order_id: 0,
            order_status: 0,
        }
    }
}

impl ros2_client::Message for RobotInfo {}
