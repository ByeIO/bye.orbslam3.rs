use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeliveryAlert {
    pub id: ::std::string::String,
    pub category: crate::rmf_fleet_msgs::msg::DeliveryAlertCategory,
    pub tier: crate::rmf_fleet_msgs::msg::DeliveryAlertTier,
    pub task_id: ::std::string::String,
    pub action: crate::rmf_fleet_msgs::msg::DeliveryAlertAction,
    pub message: ::std::string::String,
}

impl Default for DeliveryAlert {
    fn default() -> Self {
        DeliveryAlert {
            id: ::std::string::String::new(),
            category: crate::rmf_fleet_msgs::msg::DeliveryAlertCategory::default(),
            tier: crate::rmf_fleet_msgs::msg::DeliveryAlertTier::default(),
            task_id: ::std::string::String::new(),
            action: crate::rmf_fleet_msgs::msg::DeliveryAlertAction::default(),
            message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for DeliveryAlert {}
