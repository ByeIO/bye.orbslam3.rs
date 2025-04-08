use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EsrStatus7 {
    pub header: crate::std_msgs::msg::Header,
    pub canmsg: ::std::string::String,
    pub active_fault_0: u8,
    pub active_fault_1: u8,
    pub active_fault_2: u8,
    pub active_fault_3: u8,
    pub active_fault_4: u8,
    pub active_fault_5: u8,
    pub active_fault_6: u8,
    pub active_fault_7: u8,
}

impl Default for EsrStatus7 {
    fn default() -> Self {
        EsrStatus7 {
            header: crate::std_msgs::msg::Header::default(),
            canmsg: ::std::string::String::new(),
            active_fault_0: 0,
            active_fault_1: 0,
            active_fault_2: 0,
            active_fault_3: 0,
            active_fault_4: 0,
            active_fault_5: 0,
            active_fault_6: 0,
            active_fault_7: 0,
        }
    }
}

impl ros2_client::Message for EsrStatus7 {}
