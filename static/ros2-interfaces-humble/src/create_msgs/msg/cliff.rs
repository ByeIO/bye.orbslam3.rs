use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cliff {
    pub header: crate::std_msgs::msg::Header,
    pub is_cliff_left: bool,
    pub is_cliff_front_left: bool,
    pub is_cliff_right: bool,
    pub is_cliff_front_right: bool,
}

impl Default for Cliff {
    fn default() -> Self {
        Cliff {
            header: crate::std_msgs::msg::Header::default(),
            is_cliff_left: false,
            is_cliff_front_left: false,
            is_cliff_right: false,
            is_cliff_front_right: false,
        }
    }
}

impl ros2_client::Message for Cliff {}
