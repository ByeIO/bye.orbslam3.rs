use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DioPortState {
    pub header: crate::std_msgs::msg::Header,
    pub value: u64,
}

impl Default for DioPortState {
    fn default() -> Self {
        DioPortState {
            header: crate::std_msgs::msg::Header::default(),
            value: 0,
        }
    }
}

impl ros2_client::Message for DioPortState {}
