use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConfigureOutputsKpKiCmd {
    pub header: crate::std_msgs::msg::Header,
    pub confirm: bool,
    pub kp: u16,
    pub ki: u16,
}

impl Default for ConfigureOutputsKpKiCmd {
    fn default() -> Self {
        ConfigureOutputsKpKiCmd {
            header: crate::std_msgs::msg::Header::default(),
            confirm: false,
            kp: 0,
            ki: 0,
        }
    }
}

impl ros2_client::Message for ConfigureOutputsKpKiCmd {}
