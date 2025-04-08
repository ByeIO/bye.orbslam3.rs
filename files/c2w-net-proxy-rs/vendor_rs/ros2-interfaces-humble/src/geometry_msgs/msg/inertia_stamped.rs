use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InertiaStamped {
    pub header: crate::std_msgs::msg::Header,
    pub inertia: crate::geometry_msgs::msg::Inertia,
}

impl Default for InertiaStamped {
    fn default() -> Self {
        InertiaStamped {
            header: crate::std_msgs::msg::Header::default(),
            inertia: crate::geometry_msgs::msg::Inertia::default(),
        }
    }
}

impl ros2_client::Message for InertiaStamped {}
