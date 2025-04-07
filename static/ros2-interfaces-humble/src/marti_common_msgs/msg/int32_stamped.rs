use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Int32Stamped {
    pub header: crate::std_msgs::msg::Header,
    pub value: i32,
}

impl Default for Int32Stamped {
    fn default() -> Self {
        Int32Stamped {
            header: crate::std_msgs::msg::Header::default(),
            value: 0,
        }
    }
}

impl ros2_client::Message for Int32Stamped {}
