use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UBXNavDOP {
    pub header: crate::std_msgs::msg::Header,
    pub itow: u32,
    pub g_dop: u32,
    pub p_dop: u32,
    pub t_dop: u32,
    pub v_dop: u32,
    pub h_dop: u32,
    pub n_dop: u32,
    pub e_dop: u32,
}

impl Default for UBXNavDOP {
    fn default() -> Self {
        UBXNavDOP {
            header: crate::std_msgs::msg::Header::default(),
            itow: 0,
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

impl ros2_client::Message for UBXNavDOP {}
