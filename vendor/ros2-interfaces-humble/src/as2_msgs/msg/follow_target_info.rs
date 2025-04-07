use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FollowTargetInfo {
    pub header: crate::std_msgs::msg::Header,
    pub follow_status: i8,
    pub follow_mode: i8,
}

impl FollowTargetInfo {
    pub const WAITING: i8 = 0;
    pub const RUNNING: i8 = 1;
    pub const END: i8 = 2;
    pub const UNSET: i8 = 0;
    pub const PICKUP: i8 = 1;
    pub const UNPICK: i8 = 2;
    pub const DYNAMIC_LAND: i8 = 3;
    pub const DYNAMIC_FOLLOWER: i8 = 4;
}

impl Default for FollowTargetInfo {
    fn default() -> Self {
        FollowTargetInfo {
            header: crate::std_msgs::msg::Header::default(),
            follow_status: 0,
            follow_mode: 0,
        }
    }
}

impl ros2_client::Message for FollowTargetInfo {}
