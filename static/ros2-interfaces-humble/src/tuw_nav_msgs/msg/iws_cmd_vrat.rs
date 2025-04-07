use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IwsCmdVRAT {
    pub header: crate::std_msgs::msg::Header,
    pub v: f64,
    pub rho: f64,
    pub phi: f64,
    pub delta_t: f64,
}

impl Default for IwsCmdVRAT {
    fn default() -> Self {
        IwsCmdVRAT {
            header: crate::std_msgs::msg::Header::default(),
            v: 0.0,
            rho: 0.0,
            phi: 0.0,
            delta_t: 0.0,
        }
    }
}

impl ros2_client::Message for IwsCmdVRAT {}
