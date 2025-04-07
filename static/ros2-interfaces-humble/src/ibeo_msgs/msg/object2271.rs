use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Object2271 {
    pub id: u32,
    pub tracked_properties_available: bool,
    pub untracked_properties_available: bool,
    pub tracked_properties: crate::ibeo_msgs::msg::TrackedProperties,
    pub untracked_properties: crate::ibeo_msgs::msg::UntrackedProperties,
}

impl Default for Object2271 {
    fn default() -> Self {
        Object2271 {
            id: 0,
            tracked_properties_available: false,
            untracked_properties_available: false,
            tracked_properties: crate::ibeo_msgs::msg::TrackedProperties::default(),
            untracked_properties: crate::ibeo_msgs::msg::UntrackedProperties::default(),
        }
    }
}

impl ros2_client::Message for Object2271 {}
