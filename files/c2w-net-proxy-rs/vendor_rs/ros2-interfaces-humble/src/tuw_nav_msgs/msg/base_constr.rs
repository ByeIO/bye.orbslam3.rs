use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BaseConstr {
    pub header: crate::std_msgs::msg::Header,
    pub v_max: f64,
    pub av_max: f64,
    pub w_max: f64,
    pub aw_max: f64,
    pub omg_wh_max: f64,
}

impl Default for BaseConstr {
    fn default() -> Self {
        BaseConstr {
            header: crate::std_msgs::msg::Header::default(),
            v_max: 0.0,
            av_max: 0.0,
            w_max: 0.0,
            aw_max: 0.0,
            omg_wh_max: 0.0,
        }
    }
}

impl ros2_client::Message for BaseConstr {}
