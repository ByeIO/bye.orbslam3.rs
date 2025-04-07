use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestMessage {
    pub header: crate::std_msgs::msg::Header,
    pub b: bool,
    pub ui8: u8,
    pub ui16: u16,
    pub ui32: u32,
    pub ui64: u64,
    pub i8: i8,
    pub i16: i16, // default: 256
    pub i32: i32,
    pub i64: i64,
    pub f32: f32,
    pub f64: f64,
    pub str: ::std::string::String,
    pub bounded_str: ::std::string::String,
    pub t: crate::builtin_interfaces::msg::Time,
    pub d: crate::builtin_interfaces::msg::Duration,
    pub point_arr: Vec<crate::geometry_msgs::msg::Point>,
}

impl Default for TestMessage {
    fn default() -> Self {
        TestMessage {
            header: crate::std_msgs::msg::Header::default(),
            b: false,
            ui8: 0,
            ui16: 0,
            ui32: 0,
            ui64: 0,
            i8: 0,
            i16: 256,
            i32: 0,
            i64: 0,
            f32: 0.0,
            f64: 0.0,
            str: ::std::string::String::new(),
            bounded_str: ::std::string::String::new(),
            t: crate::builtin_interfaces::msg::Time::default(),
            d: crate::builtin_interfaces::msg::Duration::default(),
            point_arr: Vec::new(),
        }
    }
}

impl ros2_client::Message for TestMessage {}
