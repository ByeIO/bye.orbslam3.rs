use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Gpvtg {
    pub header: crate::std_msgs::msg::Header,
    pub message_id: ::std::string::String,
    pub track_t: f32,
    pub track_t_ref: ::std::string::String,
    pub track_m: f32,
    pub track_m_ref: ::std::string::String,
    pub speed_n: f32,
    pub speed_n_unit: ::std::string::String,
    pub speed_k: f32,
    pub speed_k_unit: ::std::string::String,
    pub mode_indicator: ::std::string::String,
}

impl Default for Gpvtg {
    fn default() -> Self {
        Gpvtg {
            header: crate::std_msgs::msg::Header::default(),
            message_id: ::std::string::String::new(),
            track_t: 0.0,
            track_t_ref: ::std::string::String::new(),
            track_m: 0.0,
            track_m_ref: ::std::string::String::new(),
            speed_n: 0.0,
            speed_n_unit: ::std::string::String::new(),
            speed_k: 0.0,
            speed_k_unit: ::std::string::String::new(),
            mode_indicator: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for Gpvtg {}
