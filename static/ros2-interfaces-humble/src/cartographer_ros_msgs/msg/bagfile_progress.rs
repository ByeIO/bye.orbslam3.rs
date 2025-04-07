use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BagfileProgress {
    pub current_bagfile_name: ::std::string::String,
    pub current_bagfile_id: u32,
    pub total_bagfiles: u32,
    pub total_messages: u32,
    pub processed_messages: u32,
    pub total_seconds: f32,
    pub processed_seconds: f32,
}

impl Default for BagfileProgress {
    fn default() -> Self {
        BagfileProgress {
            current_bagfile_name: ::std::string::String::new(),
            current_bagfile_id: 0,
            total_bagfiles: 0,
            total_messages: 0,
            processed_messages: 0,
            total_seconds: 0.0,
            processed_seconds: 0.0,
        }
    }
}

impl ros2_client::Message for BagfileProgress {}
