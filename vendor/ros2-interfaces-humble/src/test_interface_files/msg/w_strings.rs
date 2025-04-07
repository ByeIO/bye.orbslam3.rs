use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WStrings {
    pub wstring_value: ::std::string::String,
    pub wstring_value_default1: ::std::string::String, // default: "Hello world!"
    pub wstring_value_default2: ::std::string::String, // default: "Hellö wörld!"
    pub wstring_value_default3: ::std::string::String, // default: "ハローワールド"
    pub array_of_wstrings: [::std::string::String; 3],
    pub bounded_sequence_of_wstrings: Vec<::std::string::String>,
    pub unbounded_sequence_of_wstrings: Vec<::std::string::String>,
}

impl Default for WStrings {
    fn default() -> Self {
        WStrings {
            wstring_value: ::std::string::String::new(),
            wstring_value_default1: ::std::string::String::from("Hello world!"),
            wstring_value_default2: ::std::string::String::from("Hellö wörld!"),
            wstring_value_default3: ::std::string::String::from("ハローワールド"),
            array_of_wstrings: core::array::from_fn(|_| ::std::string::String::default()),
            bounded_sequence_of_wstrings: Vec::new(),
            unbounded_sequence_of_wstrings: Vec::new(),
        }
    }
}

impl ros2_client::Message for WStrings {}
