use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Object {
    pub header: crate::std_msgs::msg::Header,
    pub a: crate::off_highway_radar_msgs::msg::ObjectA,
    pub b: crate::off_highway_radar_msgs::msg::ObjectB,
}

impl Default for Object {
    fn default() -> Self {
        Object {
            header: crate::std_msgs::msg::Header::default(),
            a: crate::off_highway_radar_msgs::msg::ObjectA::default(),
            b: crate::off_highway_radar_msgs::msg::ObjectB::default(),
        }
    }
}

impl ros2_client::Message for Object {}
