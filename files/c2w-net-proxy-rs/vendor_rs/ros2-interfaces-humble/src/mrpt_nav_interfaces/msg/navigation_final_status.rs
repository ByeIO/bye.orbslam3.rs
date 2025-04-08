use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NavigationFinalStatus {
    pub navigation_status: i8,
}

impl NavigationFinalStatus {
    pub const STATUS_SUCCESS: i8 = 0;
    pub const STATUS_ERROR: i8 = 1;
    pub const STATUS_CANCELLED: i8 = 2;
}

impl Default for NavigationFinalStatus {
    fn default() -> Self {
        NavigationFinalStatus {
            navigation_status: 0,
        }
    }
}

impl ros2_client::Message for NavigationFinalStatus {}
