use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaskType {
    #[serde(rename = "type")]    pub type_: u32,
}

impl TaskType {
    pub const TYPE_STATION: u32 = 0;
    pub const TYPE_LOOP: u32 = 1;
    pub const TYPE_DELIVERY: u32 = 2;
    pub const TYPE_CHARGE_BATTERY: u32 = 3;
    pub const TYPE_CLEAN: u32 = 4;
    pub const TYPE_PATROL: u32 = 5;
}

impl Default for TaskType {
    fn default() -> Self {
        TaskType {
            type_: 0,
        }
    }
}

impl ros2_client::Message for TaskType {}
