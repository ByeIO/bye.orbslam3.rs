use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DispatchAck {
    pub dispatch_id: u64,
    pub success: bool,
    pub errors: Vec<::std::string::String>,
}

impl Default for DispatchAck {
    fn default() -> Self {
        DispatchAck {
            dispatch_id: 0,
            success: false,
            errors: Vec::new(),
        }
    }
}

impl ros2_client::Message for DispatchAck {}
