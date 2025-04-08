use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SbgGpsHdt {
    pub header: crate::std_msgs::msg::Header,
    pub time_stamp: u32,
    pub status: u16,
    pub tow: u32,
    pub true_heading: f32,
    pub true_heading_acc: f32,
    pub pitch: f32,
    pub pitch_acc: f32,
    pub baseline: f32,
    pub num_sv_tracked: u8,
    pub num_sv_used: u8,
}

impl Default for SbgGpsHdt {
    fn default() -> Self {
        SbgGpsHdt {
            header: crate::std_msgs::msg::Header::default(),
            time_stamp: 0,
            status: 0,
            tow: 0,
            true_heading: 0.0,
            true_heading_acc: 0.0,
            pitch: 0.0,
            pitch_acc: 0.0,
            baseline: 0.0,
            num_sv_tracked: 0,
            num_sv_used: 0,
        }
    }
}

impl ros2_client::Message for SbgGpsHdt {}
