use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Float64Stamped {
    pub header: crate::std_msgs::msg::Header,
    pub value: f64,
}

impl Default for Float64Stamped {
    fn default() -> Self {
        Float64Stamped {
            header: crate::std_msgs::msg::Header::default(),
            value: 0.0,
        }
    }
}

impl ros2_client::Message for Float64Stamped {}
