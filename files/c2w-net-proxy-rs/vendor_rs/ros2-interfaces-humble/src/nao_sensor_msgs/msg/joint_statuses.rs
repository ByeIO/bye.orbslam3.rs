use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JointStatuses {
    pub statuses: [i32; 25],
}

impl JointStatuses {
    pub const STATUS_NORMAL: i32 = 0;
    pub const STATUS_HOT: i32 = 1;
    pub const STATUS_VERY_HOT: i32 = 2;
    pub const STATUS_CRITICALLY_HOT: i32 = 3;
}

impl Default for JointStatuses {
    fn default() -> Self {
        JointStatuses {
            statuses: [0; 25],
        }
    }
}

impl ros2_client::Message for JointStatuses {}
