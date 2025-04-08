use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CfgDAT {
    pub datum_num: u16,
    pub datum_name: [u8; 6],
    pub maj_a: f64,
    pub flat: f64,
    pub d_x: f32,
    pub d_y: f32,
    pub d_z: f32,
    pub rot_x: f32,
    pub rot_y: f32,
    pub rot_z: f32,
    pub scale: f32,
}

impl CfgDAT {
    pub const CLASS_ID: u8 = 6;
    pub const MESSAGE_ID: u8 = 6;
    pub const DATUM_NUM_WGS84: u16 = 0;
    pub const DATUM_NUM_USER: u16 = 65535;
}

impl Default for CfgDAT {
    fn default() -> Self {
        CfgDAT {
            datum_num: 0,
            datum_name: [0; 6],
            maj_a: 0.0,
            flat: 0.0,
            d_x: 0.0,
            d_y: 0.0,
            d_z: 0.0,
            rot_x: 0.0,
            rot_y: 0.0,
            rot_z: 0.0,
            scale: 0.0,
        }
    }
}

impl ros2_client::Message for CfgDAT {}
