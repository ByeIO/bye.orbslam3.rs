use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IwsCmdVRATVec {
    pub header: crate::std_msgs::msg::Header,
    pub v: Vec<f64>,
    pub rho: Vec<f64>,
    pub phi: Vec<f64>,
    pub delta_t: Vec<f64>,
    pub state0: Vec<f64>,
}

impl Default for IwsCmdVRATVec {
    fn default() -> Self {
        IwsCmdVRATVec {
            header: crate::std_msgs::msg::Header::default(),
            v: Vec::new(),
            rho: Vec::new(),
            phi: Vec::new(),
            delta_t: Vec::new(),
            state0: Vec::new(),
        }
    }
}

impl ros2_client::Message for IwsCmdVRATVec {}
