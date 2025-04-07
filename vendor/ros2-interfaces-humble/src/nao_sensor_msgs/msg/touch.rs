use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Touch {
    pub head_front: bool,
    pub head_middle: bool,
    pub head_rear: bool,
}

impl Default for Touch {
    fn default() -> Self {
        Touch {
            head_front: false,
            head_middle: false,
            head_rear: false,
        }
    }
}

impl ros2_client::Message for Touch {}
