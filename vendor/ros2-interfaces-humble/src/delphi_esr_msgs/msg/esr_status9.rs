use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EsrStatus9 {
    pub header: crate::std_msgs::msg::Header,
    pub canmsg: ::std::string::String,
    pub avg_pwr_cwblkg: u16,
    pub sideslip_angle: f32,
    pub serial_num_3rd_byte: u8,
    pub water_spray_target_id: u8,
    pub filtered_xohp_acc_cipv: f32,
    pub path_id_acc_2: u8,
    pub path_id_acc_3: u8,
}

impl Default for EsrStatus9 {
    fn default() -> Self {
        EsrStatus9 {
            header: crate::std_msgs::msg::Header::default(),
            canmsg: ::std::string::String::new(),
            avg_pwr_cwblkg: 0,
            sideslip_angle: 0.0,
            serial_num_3rd_byte: 0,
            water_spray_target_id: 0,
            filtered_xohp_acc_cipv: 0.0,
            path_id_acc_2: 0,
            path_id_acc_3: 0,
        }
    }
}

impl ros2_client::Message for EsrStatus9 {}
