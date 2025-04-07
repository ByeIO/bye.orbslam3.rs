use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NavCLOCK {
    pub i_tow: u32,
    pub clk_b: i32,
    pub clk_d: i32,
    pub t_acc: u32,
    pub f_acc: u32,
}

impl NavCLOCK {
    pub const CLASS_ID: u8 = 1;
    pub const MESSAGE_ID: u8 = 34;
}

impl Default for NavCLOCK {
    fn default() -> Self {
        NavCLOCK {
            i_tow: 0,
            clk_b: 0,
            clk_d: 0,
            t_acc: 0,
            f_acc: 0,
        }
    }
}

impl ros2_client::Message for NavCLOCK {}
