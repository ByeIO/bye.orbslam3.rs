use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RxmRAWSV {
    pub cp_mes: f64,
    pub pr_mes: f64,
    pub do_mes: f32,
    pub sv: u8,
    pub mes_qi: i8,
    pub cno: i8,
    pub lli: u8,
}

impl Default for RxmRAWSV {
    fn default() -> Self {
        RxmRAWSV {
            cp_mes: 0.0,
            pr_mes: 0.0,
            do_mes: 0.0,
            sv: 0,
            mes_qi: 0,
            cno: 0,
            lli: 0,
        }
    }
}

impl ros2_client::Message for RxmRAWSV {}
