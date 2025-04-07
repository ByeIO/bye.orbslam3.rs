use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParamVec {
    pub header: crate::std_msgs::msg::Header,
    pub params: Vec<crate::rcl_interfaces::msg::Parameter>,
}

impl Default for ParamVec {
    fn default() -> Self {
        ParamVec {
            header: crate::std_msgs::msg::Header::default(),
            params: Vec::new(),
        }
    }
}

impl ros2_client::Message for ParamVec {}
