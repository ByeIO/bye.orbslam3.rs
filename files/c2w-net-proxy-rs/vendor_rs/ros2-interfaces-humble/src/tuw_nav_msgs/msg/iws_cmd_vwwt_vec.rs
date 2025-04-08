use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IwsCmdVWWTVec {
    pub header: crate::std_msgs::msg::Header,
    pub v: Vec<f64>,
    pub w_f: Vec<f64>,
    pub w_icc: Vec<f64>,
    pub delta_t: Vec<f64>,
    pub state0: Vec<f64>,
}

impl Default for IwsCmdVWWTVec {
    fn default() -> Self {
        IwsCmdVWWTVec {
            header: crate::std_msgs::msg::Header::default(),
            v: Vec::new(),
            w_f: Vec::new(),
            w_icc: Vec::new(),
            delta_t: Vec::new(),
            state0: Vec::new(),
        }
    }
}

impl ros2_client::Message for IwsCmdVWWTVec {}
