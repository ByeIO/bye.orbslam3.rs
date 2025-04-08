use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiffDriveCmdVWVec {
    pub header: crate::std_msgs::msg::Header,
    pub v: Vec<f64>,
    pub w: Vec<f64>,
    pub delta_t: Vec<f64>,
    pub state0: Vec<f64>,
}

impl Default for DiffDriveCmdVWVec {
    fn default() -> Self {
        DiffDriveCmdVWVec {
            header: crate::std_msgs::msg::Header::default(),
            v: Vec::new(),
            w: Vec::new(),
            delta_t: Vec::new(),
            state0: Vec::new(),
        }
    }
}

impl ros2_client::Message for DiffDriveCmdVWVec {}
