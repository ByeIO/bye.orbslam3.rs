use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InterfaceButtons {
    pub header: crate::std_msgs::msg::Header,
    pub button_1: crate::irobot_create_msgs::msg::Button,
    pub button_power: crate::irobot_create_msgs::msg::Button,
    pub button_2: crate::irobot_create_msgs::msg::Button,
}

impl Default for InterfaceButtons {
    fn default() -> Self {
        InterfaceButtons {
            header: crate::std_msgs::msg::Header::default(),
            button_1: crate::irobot_create_msgs::msg::Button::default(),
            button_power: crate::irobot_create_msgs::msg::Button::default(),
            button_2: crate::irobot_create_msgs::msg::Button::default(),
        }
    }
}

impl ros2_client::Message for InterfaceButtons {}
