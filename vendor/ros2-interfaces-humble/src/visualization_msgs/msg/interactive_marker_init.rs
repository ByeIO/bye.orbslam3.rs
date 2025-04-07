use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InteractiveMarkerInit {
    pub server_id: ::std::string::String,
    pub seq_num: u64,
    pub markers: Vec<crate::visualization_msgs::msg::InteractiveMarker>,
}

impl Default for InteractiveMarkerInit {
    fn default() -> Self {
        InteractiveMarkerInit {
            server_id: ::std::string::String::new(),
            seq_num: 0,
            markers: Vec::new(),
        }
    }
}

impl ros2_client::Message for InteractiveMarkerInit {}
