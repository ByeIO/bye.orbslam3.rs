use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StringArrays {
    pub ub_string_static_array_value: [::std::string::String; 3],
    pub ub_string_ub_array_value: Vec<::std::string::String>,
    pub ub_string_dynamic_array_value: Vec<::std::string::String>,
    pub string_dynamic_array_value: Vec<::std::string::String>,
    pub string_static_array_value: [::std::string::String; 3],
    pub string_bounded_array_value: Vec<::std::string::String>,
    pub def_string_dynamic_array_value: Vec<::std::string::String>, // default: ["What", "a", "wonderful", "world", "!"]
    pub def_string_static_array_value: [::std::string::String; 3], // default: ["Hello", "World", "!"]
    pub def_string_bounded_array_value: Vec<::std::string::String>, // default: ['Hello', 'World', "!"]
    pub def_various_quotes: Vec<::std::string::String>, // default: ["H\"el'lo", 'Wo\'r"ld']
    pub def_various_commas: Vec<::std::string::String>, // default: ["Hel,lo", ',World', abcd , "!,",]
}

impl Default for StringArrays {
    fn default() -> Self {
        StringArrays {
            ub_string_static_array_value: core::array::from_fn(|_| ::std::string::String::default()),
            ub_string_ub_array_value: Vec::new(),
            ub_string_dynamic_array_value: Vec::new(),
            string_dynamic_array_value: Vec::new(),
            string_static_array_value: core::array::from_fn(|_| ::std::string::String::default()),
            string_bounded_array_value: Vec::new(),
            def_string_dynamic_array_value: vec![::std::string::String::from("What"), ::std::string::String::from("a"), ::std::string::String::from("wonderful"), ::std::string::String::from("world"), ::std::string::String::from("!")],
            def_string_static_array_value: [::std::string::String::from("Hello"), ::std::string::String::from("World"), ::std::string::String::from("!")],
            def_string_bounded_array_value: vec![::std::string::String::from("Hello"), ::std::string::String::from("World"), ::std::string::String::from("!")],
            def_various_quotes: vec![::std::string::String::from("H\\\"el'lo"), ::std::string::String::from("Wo\\'r\"ld")],
            def_various_commas: vec![::std::string::String::from("Hel"), ::std::string::String::from("lo"), ::std::string::String::from(""), ::std::string::String::from("World"), ::std::string::String::from("abcd"), ::std::string::String::from("!"), ::std::string::String::from(""), ::std::string::String::from("")],
        }
    }
}

impl ros2_client::Message for StringArrays {}
