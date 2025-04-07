use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Objects {
    pub header: crate::std_msgs::msg::Header,
    pub objects: Vec<crate::off_highway_uss_msgs::msg::Object>,
}

impl Default for Objects {
    fn default() -> Self {
        Objects {
            header: crate::std_msgs::msg::Header::default(),
            objects: Vec::new(),
        }
    }
}

impl ros2_client::Message for Objects {}
