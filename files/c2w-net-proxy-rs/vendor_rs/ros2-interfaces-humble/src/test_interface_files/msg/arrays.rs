use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Arrays {
    pub bool_values: [bool; 3],
    pub byte_values: [u8; 3],
    pub char_values: [i8; 3],
    pub float32_values: [f32; 3],
    pub float64_values: [f64; 3],
    pub int8_values: [i8; 3],
    pub uint8_values: [u8; 3],
    pub int16_values: [i16; 3],
    pub uint16_values: [u16; 3],
    pub int32_values: [i32; 3],
    pub uint32_values: [u32; 3],
    pub int64_values: [i64; 3],
    pub uint64_values: [u64; 3],
    pub string_values: [::std::string::String; 3],
    pub basic_types_values: [crate::test_interface_files::msg::BasicTypes; 3],
    pub constants_values: [crate::test_interface_files::msg::Constants; 3],
    pub defaults_values: [crate::test_interface_files::msg::Defaults; 3],
    pub bool_values_default: [bool; 3], // default: [false, true, false]
    pub byte_values_default: [u8; 3], // default: [0, 1, 255]
    pub char_values_default: [i8; 3], // default: [0, 1, 127]
    pub float32_values_default: [f32; 3], // default: [1.125, 0.0, -1.125]
    pub float64_values_default: [f64; 3], // default: [3.1415, 0.0, -3.1415]
    pub int8_values_default: [i8; 3], // default: [0, 127, -128]
    pub uint8_values_default: [u8; 3], // default: [0, 1, 255]
    pub int16_values_default: [i16; 3], // default: [0, 32767, -32768]
    pub uint16_values_default: [u16; 3], // default: [0, 1, 65535]
    pub int32_values_default: [i32; 3], // default: [0, 2147483647, -2147483648]
    pub uint32_values_default: [u32; 3], // default: [0, 1, 4294967295]
    pub int64_values_default: [i64; 3], // default: [0, 9223372036854775807, -9223372036854775808]
    pub uint64_values_default: [u64; 3], // default: [0, 1, 18446744073709551615]
    pub string_values_default: [::std::string::String; 3], // default: ["", "max value", "min value"]
    pub alignment_check: i32,
}

impl Default for Arrays {
    fn default() -> Self {
        Arrays {
            bool_values: [false; 3],
            byte_values: [0; 3],
            char_values: [0; 3],
            float32_values: [0.0; 3],
            float64_values: [0.0; 3],
            int8_values: [0; 3],
            uint8_values: [0; 3],
            int16_values: [0; 3],
            uint16_values: [0; 3],
            int32_values: [0; 3],
            uint32_values: [0; 3],
            int64_values: [0; 3],
            uint64_values: [0; 3],
            string_values: core::array::from_fn(|_| ::std::string::String::default()),
            basic_types_values: core::array::from_fn(|_| crate::test_interface_files::msg::BasicTypes::default()),
            constants_values: core::array::from_fn(|_| crate::test_interface_files::msg::Constants::default()),
            defaults_values: core::array::from_fn(|_| crate::test_interface_files::msg::Defaults::default()),
            bool_values_default: [false, true, false],
            byte_values_default: [0, 1, 255],
            char_values_default: [0, 1, 127],
            float32_values_default: [1.125, 0.0, -1.125],
            float64_values_default: [3.1415, 0.0, -3.1415],
            int8_values_default: [0, 127, -128],
            uint8_values_default: [0, 1, 255],
            int16_values_default: [0, 32767, -32768],
            uint16_values_default: [0, 1, 65535],
            int32_values_default: [0, 2147483647, -2147483648],
            uint32_values_default: [0, 1, 4294967295],
            int64_values_default: [0, 9223372036854775807, -9223372036854775808],
            uint64_values_default: [0, 1, 18446744073709551615],
            string_values_default: [::std::string::String::from(""), ::std::string::String::from("max value"), ::std::string::String::from("min value")],
            alignment_check: 0,
        }
    }
}

impl ros2_client::Message for Arrays {}
