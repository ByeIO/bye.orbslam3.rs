use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RouterStatus {
    pub id: i32,
    pub success: bool,
    pub missing_robots: Vec<::std::string::String>,
    pub duration: i32,
    pub overall_path_length: i32,
    pub longest_path_length: i32,
    pub priority_scheduling_attemps: i32,
    pub speed_scheduling_attemps: i32,
}

impl Default for RouterStatus {
    fn default() -> Self {
        RouterStatus {
            id: 0,
            success: false,
            missing_robots: Vec::new(),
            duration: 0,
            overall_path_length: 0,
            longest_path_length: 0,
            priority_scheduling_attemps: 0,
            speed_scheduling_attemps: 0,
        }
    }
}

impl ros2_client::Message for RouterStatus {}
