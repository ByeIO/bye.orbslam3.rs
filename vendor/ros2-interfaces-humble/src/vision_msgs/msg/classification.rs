use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Classification {
    pub header: crate::std_msgs::msg::Header,
    pub results: Vec<crate::vision_msgs::msg::ObjectHypothesis>,
}

impl Default for Classification {
    fn default() -> Self {
        Classification {
            header: crate::std_msgs::msg::Header::default(),
            results: Vec::new(),
        }
    }
}

impl ros2_client::Message for Classification {}
