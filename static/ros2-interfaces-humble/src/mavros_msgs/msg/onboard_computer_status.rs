use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OnboardComputerStatus {
    pub header: crate::std_msgs::msg::Header,
    pub component: u8,
    pub uptime: u32,
    #[serde(rename = "type")]    pub type_: u8,
    pub cpu_cores: [u8; 8],
    pub cpu_combined: [u8; 10],
    pub gpu_cores: [u8; 4],
    pub gpu_combined: [u8; 10],
    pub temperature_board: i8,
    pub temperature_core: [i8; 8],
    pub fan_speed: [i16; 4],
    pub ram_usage: u32,
    pub ram_total: u32,
    pub storage_type: [u32; 4],
    pub storage_usage: [u32; 4],
    pub storage_total: [u32; 4],
    pub link_type: [u32; 6],
    pub link_tx_rate: [u32; 6],
    pub link_rx_rate: [u32; 6],
    pub link_tx_max: [u32; 6],
    pub link_rx_max: [u32; 6],
}

impl Default for OnboardComputerStatus {
    fn default() -> Self {
        OnboardComputerStatus {
            header: crate::std_msgs::msg::Header::default(),
            component: 0,
            uptime: 0,
            type_: 0,
            cpu_cores: [0; 8],
            cpu_combined: [0; 10],
            gpu_cores: [0; 4],
            gpu_combined: [0; 10],
            temperature_board: 0,
            temperature_core: [0; 8],
            fan_speed: [0; 4],
            ram_usage: 0,
            ram_total: 0,
            storage_type: [0; 4],
            storage_usage: [0; 4],
            storage_total: [0; 4],
            link_type: [0; 6],
            link_tx_rate: [0; 6],
            link_rx_rate: [0; 6],
            link_tx_max: [0; 6],
            link_rx_max: [0; 6],
        }
    }
}

impl ros2_client::Message for OnboardComputerStatus {}
