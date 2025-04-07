use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnboundedSequences {
    pub bool_values: Vec<bool>,
    pub byte_values: Vec<u8>,
    pub char_values: Vec<i8>,
    pub float32_values: Vec<f32>,
    pub float64_values: Vec<f64>,
    pub int8_values: Vec<i8>,
    pub uint8_values: Vec<u8>,
    pub int16_values: Vec<i16>,
    pub uint16_values: Vec<u16>,
    pub int32_values: Vec<i32>,
    pub uint32_values: Vec<u32>,
    pub int64_values: Vec<i64>,
    pub uint64_values: Vec<u64>,
    pub string_values: Vec<::std::string::String>,
    pub basic_types_values: Vec<crate::test_interface_files::msg::BasicTypes>,
    pub constants_values: Vec<crate::test_interface_files::msg::Constants>,
    pub defaults_values: Vec<crate::test_interface_files::msg::Defaults>,
    pub bool_values_default: Vec<bool>, // default: [false, true, false]
    pub byte_values_default: Vec<u8>, // default: [0, 1, 255]
    pub char_values_default: Vec<i8>, // default: [0, 1, 127]
    pub float32_values_default: Vec<f32>, // default: [1.125, 0.0, -1.125]
    pub float64_values_default: Vec<f64>, // default: [3.1415, 0.0, -3.1415]
    pub int8_values_default: Vec<i8>, // default: [0, 127, -128]
    pub uint8_values_default: Vec<u8>, // default: [0, 1, 255]
    pub int16_values_default: Vec<i16>, // default: [0, 32767, -32768]
    pub uint16_values_default: Vec<u16>, // default: [0, 1, 65535]
    pub int32_values_default: Vec<i32>, // default: [0, 2147483647, -2147483648]
    pub uint32_values_default: Vec<u32>, // default: [0, 1, 4294967295]
    pub int64_values_default: Vec<i64>, // default: [0, 9223372036854775807, -9223372036854775808]
    pub uint64_values_default: Vec<u64>, // default: [0, 1, 18446744073709551615]
    pub string_values_default: Vec<::std::string::String>, // default: ["", "max value", "min value"]
    pub alignment_check: i32,
}

impl Default for UnboundedSequences {
    fn default() -> Self {
        UnboundedSequences {
            bool_values: Vec::new(),
            byte_values: Vec::new(),
            char_values: Vec::new(),
            float32_values: Vec::new(),
            float64_values: Vec::new(),
            int8_values: Vec::new(),
            uint8_values: Vec::new(),
            int16_values: Vec::new(),
            uint16_values: Vec::new(),
            int32_values: Vec::new(),
            uint32_values: Vec::new(),
            int64_values: Vec::new(),
            uint64_values: Vec::new(),
            string_values: Vec::new(),
            basic_types_values: Vec::new(),
            constants_values: Vec::new(),
            defaults_values: Vec::new(),
            bool_values_default: vec![false, true, false],
            byte_values_default: vec![0, 1, 255],
            char_values_default: vec![0, 1, 127],
            float32_values_default: vec![1.125, 0.0, -1.125],
            float64_values_default: vec![3.1415, 0.0, -3.1415],
            int8_values_default: vec![0, 127, -128],
            uint8_values_default: vec![0, 1, 255],
            int16_values_default: vec![0, 32767, -32768],
            uint16_values_default: vec![0, 1, 65535],
            int32_values_default: vec![0, 2147483647, -2147483648],
            uint32_values_default: vec![0, 1, 4294967295],
            int64_values_default: vec![0, 9223372036854775807, -9223372036854775808],
            uint64_values_default: vec![0, 1, 18446744073709551615],
            string_values_default: vec![::std::string::String::from(""), ::std::string::String::from("max value"), ::std::string::String::from("min value")],
            alignment_check: 0,
        }
    }
}

impl ros2_client::Message for UnboundedSequences {}
