use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Dataframe {
    pub header: crate::std_msgs::msg::Header,
    pub src_address: ::std::string::String,
    pub dst_address: ::std::string::String,
    pub data: Vec<u8>,
    pub rssi: f64,
}

impl Default for Dataframe {
    fn default() -> Self {
        Dataframe {
            header: crate::std_msgs::msg::Header::default(),
            src_address: ::std::string::String::new(),
            dst_address: ::std::string::String::new(),
            data: Vec::new(),
            rssi: 0.0,
        }
    }
}

impl ros2_client::Message for Dataframe {}
