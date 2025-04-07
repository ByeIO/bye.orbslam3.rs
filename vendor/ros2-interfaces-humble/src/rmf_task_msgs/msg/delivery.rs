use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Delivery {
    pub task_id: ::std::string::String,
    pub items: Vec<crate::rmf_dispenser_msgs::msg::DispenserRequestItem>,
    pub pickup_place_name: ::std::string::String,
    pub pickup_dispenser: ::std::string::String,
    pub pickup_behavior: crate::rmf_task_msgs::msg::Behavior,
    pub dropoff_place_name: ::std::string::String,
    pub dropoff_ingestor: ::std::string::String,
    pub dropoff_behavior: crate::rmf_task_msgs::msg::Behavior,
}

impl Default for Delivery {
    fn default() -> Self {
        Delivery {
            task_id: ::std::string::String::new(),
            items: Vec::new(),
            pickup_place_name: ::std::string::String::new(),
            pickup_dispenser: ::std::string::String::new(),
            pickup_behavior: crate::rmf_task_msgs::msg::Behavior::default(),
            dropoff_place_name: ::std::string::String::new(),
            dropoff_ingestor: ::std::string::String::new(),
            dropoff_behavior: crate::rmf_task_msgs::msg::Behavior::default(),
        }
    }
}

impl ros2_client::Message for Delivery {}
