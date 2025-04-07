use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JointWrench {
    pub header: crate::std_msgs::msg::Header,
    pub body_1_name: crate::std_msgs::msg::String,
    pub body_1_id: crate::std_msgs::msg::UInt32,
    pub body_2_name: crate::std_msgs::msg::String,
    pub body_2_id: crate::std_msgs::msg::UInt32,
    pub body_1_wrench: crate::geometry_msgs::msg::Wrench,
    pub body_2_wrench: crate::geometry_msgs::msg::Wrench,
}

impl Default for JointWrench {
    fn default() -> Self {
        JointWrench {
            header: crate::std_msgs::msg::Header::default(),
            body_1_name: crate::std_msgs::msg::String::default(),
            body_1_id: crate::std_msgs::msg::UInt32::default(),
            body_2_name: crate::std_msgs::msg::String::default(),
            body_2_id: crate::std_msgs::msg::UInt32::default(),
            body_1_wrench: crate::geometry_msgs::msg::Wrench::default(),
            body_2_wrench: crate::geometry_msgs::msg::Wrench::default(),
        }
    }
}

impl ros2_client::Message for JointWrench {}
