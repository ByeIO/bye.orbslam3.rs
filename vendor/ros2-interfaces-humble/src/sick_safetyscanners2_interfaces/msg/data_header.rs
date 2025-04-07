use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataHeader {
    pub version_version: u8,
    pub version_major_version: u8,
    pub version_minor_version: u8,
    pub version_release: u8,
    pub serial_number_of_device: u32,
    pub serial_number_of_channel_plug: u32,
    pub channel_number: u8,
    pub sequence_number: u32,
    pub scan_number: u32,
    pub timestamp_date: u16,
    pub timestamp_time: u32,
}

impl Default for DataHeader {
    fn default() -> Self {
        DataHeader {
            version_version: 0,
            version_major_version: 0,
            version_minor_version: 0,
            version_release: 0,
            serial_number_of_device: 0,
            serial_number_of_channel_plug: 0,
            channel_number: 0,
            sequence_number: 0,
            scan_number: 0,
            timestamp_date: 0,
            timestamp_time: 0,
        }
    }
}

impl ros2_client::Message for DataHeader {}
