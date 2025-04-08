use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EsrStatus8 {
    pub header: crate::std_msgs::msg::Header,
    pub canmsg: ::std::string::String,
    pub history_fault_0: u8,
    pub history_fault_1: u8,
    pub history_fault_2: u8,
    pub history_fault_3: u8,
    pub history_fault_4: u8,
    pub history_fault_5: u8,
    pub history_fault_6: u8,
    pub history_fault_7: u8,
}

impl Default for EsrStatus8 {
    fn default() -> Self {
        EsrStatus8 {
            header: crate::std_msgs::msg::Header::default(),
            canmsg: ::std::string::String::new(),
            history_fault_0: 0,
            history_fault_1: 0,
            history_fault_2: 0,
            history_fault_3: 0,
            history_fault_4: 0,
            history_fault_5: 0,
            history_fault_6: 0,
            history_fault_7: 0,
        }
    }
}

impl ros2_client::Message for EsrStatus8 {}
