use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnalogIn {
    pub header: crate::std_msgs::msg::Header,
    pub command: u8,
    pub analog_data_ch4_low_byte: u8,
    pub analog_data_ch4_high_bits: u8,
    pub analog_data_ch3_low_byte: u8,
    pub analog_data_ch3_high_bits: u8,
    pub analog_data_ch2_low_byte: u8,
    pub analog_data_ch2_high_bits: u8,
    pub analog_data_ch1_low_byte: u8,
    pub analog_data_ch1_high_bits: u8,
}

impl Default for AnalogIn {
    fn default() -> Self {
        AnalogIn {
            header: crate::std_msgs::msg::Header::default(),
            command: 0,
            analog_data_ch4_low_byte: 0,
            analog_data_ch4_high_bits: 0,
            analog_data_ch3_low_byte: 0,
            analog_data_ch3_high_bits: 0,
            analog_data_ch2_low_byte: 0,
            analog_data_ch2_high_bits: 0,
            analog_data_ch1_low_byte: 0,
            analog_data_ch1_high_bits: 0,
        }
    }
}

impl ros2_client::Message for AnalogIn {}
