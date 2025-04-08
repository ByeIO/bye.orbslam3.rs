use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NavDOP {
    pub i_tow: u32,
    pub g_dop: u16,
    pub p_dop: u16,
    pub t_dop: u16,
    pub v_dop: u16,
    pub h_dop: u16,
    pub n_dop: u16,
    pub e_dop: u16,
}

impl NavDOP {
    pub const CLASS_ID: u8 = 1;
    pub const MESSAGE_ID: u8 = 4;
}

impl Default for NavDOP {
    fn default() -> Self {
        NavDOP {
            i_tow: 0,
            g_dop: 0,
            p_dop: 0,
            t_dop: 0,
            v_dop: 0,
            h_dop: 0,
            n_dop: 0,
            e_dop: 0,
        }
    }
}

impl ros2_client::Message for NavDOP {}
