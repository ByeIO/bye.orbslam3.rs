use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PVTCartesian {
    pub header: crate::std_msgs::msg::Header,
    pub block_header: crate::septentrio_gnss_driver::msg::BlockHeader,
    pub mode: u8,
    pub error: u8,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub undulation: f32,
    pub vx: f32,
    pub vy: f32,
    pub vz: f32,
    pub cog: f32,
    pub rx_clk_bias: f64,
    pub rx_clk_drift: f32,
    pub time_system: u8,
    pub datum: u8,
    pub nr_sv: u8,
    pub wa_corr_info: u8,
    pub reference_id: u16,
    pub mean_corr_age: u16,
    pub signal_info: u32,
    pub alert_flag: u8,
    pub nr_bases: u8,
    pub ppp_info: u16,
    pub latency: u16,
    pub h_accuracy: u16,
    pub v_accuracy: u16,
    pub misc: u8,
}

impl Default for PVTCartesian {
    fn default() -> Self {
        PVTCartesian {
            header: crate::std_msgs::msg::Header::default(),
            block_header: crate::septentrio_gnss_driver::msg::BlockHeader::default(),
            mode: 0,
            error: 0,
            x: 0.0,
            y: 0.0,
            z: 0.0,
            undulation: 0.0,
            vx: 0.0,
            vy: 0.0,
            vz: 0.0,
            cog: 0.0,
            rx_clk_bias: 0.0,
            rx_clk_drift: 0.0,
            time_system: 0,
            datum: 0,
            nr_sv: 0,
            wa_corr_info: 0,
            reference_id: 0,
            mean_corr_age: 0,
            signal_info: 0,
            alert_flag: 0,
            nr_bases: 0,
            ppp_info: 0,
            latency: 0,
            h_accuracy: 0,
            v_accuracy: 0,
            misc: 0,
        }
    }
}

impl ros2_client::Message for PVTCartesian {}
