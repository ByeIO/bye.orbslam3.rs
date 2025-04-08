use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommsPortInfo {
    pub port_id: u16,
    pub tx_pending: u16,
    pub tx_bytes: u32,
    pub tx_usage: u8,
    pub tx_peak_usage: u8,
    pub rx_pending: u16,
    pub rx_bytes: u32,
    pub rx_usage: u8,
    pub rx_peak_usage: u8,
    pub overrun_errs: u16,
    pub msgs: [u16; 4],
    pub skipped: u32,
}

impl Default for CommsPortInfo {
    fn default() -> Self {
        CommsPortInfo {
            port_id: 0,
            tx_pending: 0,
            tx_bytes: 0,
            tx_usage: 0,
            tx_peak_usage: 0,
            rx_pending: 0,
            rx_bytes: 0,
            rx_usage: 0,
            rx_peak_usage: 0,
            overrun_errs: 0,
            msgs: [0; 4],
            skipped: 0,
        }
    }
}

impl ros2_client::Message for CommsPortInfo {}
