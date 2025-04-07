use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VehicleInfo {
    pub header: crate::std_msgs::msg::Header,
    pub available_info: u8,
    pub sysid: u8,
    pub compid: u8,
    pub autopilot: u8,
    #[serde(rename = "type")]    pub type_: u8,
    pub system_status: u8,
    pub base_mode: u8,
    pub custom_mode: u32,
    pub mode: ::std::string::String,
    pub mode_id: u32,
    pub capabilities: u64,
    pub flight_sw_version: u32,
    pub middleware_sw_version: u32,
    pub os_sw_version: u32,
    pub board_version: u32,
    pub flight_custom_version: ::std::string::String,
    pub vendor_id: u16,
    pub product_id: u16,
    pub uid: u64,
}

impl VehicleInfo {
    pub const HAVE_INFO_HEARTBEAT: u8 = 1;
    pub const HAVE_INFO_AUTOPILOT_VERSION: u8 = 2;
}

impl Default for VehicleInfo {
    fn default() -> Self {
        VehicleInfo {
            header: crate::std_msgs::msg::Header::default(),
            available_info: 0,
            sysid: 0,
            compid: 0,
            autopilot: 0,
            type_: 0,
            system_status: 0,
            base_mode: 0,
            custom_mode: 0,
            mode: ::std::string::String::new(),
            mode_id: 0,
            capabilities: 0,
            flight_sw_version: 0,
            middleware_sw_version: 0,
            os_sw_version: 0,
            board_version: 0,
            flight_custom_version: ::std::string::String::new(),
            vendor_id: 0,
            product_id: 0,
            uid: 0,
        }
    }
}

impl ros2_client::Message for VehicleInfo {}
