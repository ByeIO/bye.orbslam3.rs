use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Strings {
    pub string_value: ::std::string::String,
    pub string_value_default1: ::std::string::String, // default: "Hello world!"
    pub string_value_default2: ::std::string::String, // default: "Hello'world!"
    pub string_value_default3: ::std::string::String, // default: 'Hello"world!'
    pub string_value_default4: ::std::string::String, // default: 'Hello\'world!'
    pub string_value_default5: ::std::string::String, // default: "Hello\"world!"
    pub bounded_string_value: ::std::string::String,
    pub bounded_string_value_default1: ::std::string::String, // default: "Hello world!"
    pub bounded_string_value_default2: ::std::string::String, // default: "Hello'world!"
    pub bounded_string_value_default3: ::std::string::String, // default: 'Hello"world!'
    pub bounded_string_value_default4: ::std::string::String, // default: 'Hello\'world!'
    pub bounded_string_value_default5: ::std::string::String, // default: "Hello\"world!"
}

impl Strings {
    pub const STRING_CONST: &'static str = "Hello world!";
}

impl Default for Strings {
    fn default() -> Self {
        Strings {
            string_value: ::std::string::String::new(),
            string_value_default1: ::std::string::String::from("Hello world!"),
            string_value_default2: ::std::string::String::from("Hello'world!"),
            string_value_default3: ::std::string::String::from("Hello\"world!"),
            string_value_default4: ::std::string::String::from("Hello\\'world!"),
            string_value_default5: ::std::string::String::from("Hello\\\"world!"),
            bounded_string_value: ::std::string::String::new(),
            bounded_string_value_default1: ::std::string::String::from("Hello world!"),
            bounded_string_value_default2: ::std::string::String::from("Hello'world!"),
            bounded_string_value_default3: ::std::string::String::from("Hello\"world!"),
            bounded_string_value_default4: ::std::string::String::from("Hello\\'world!"),
            bounded_string_value_default5: ::std::string::String::from("Hello\\\"world!"),
        }
    }
}

impl ros2_client::Message for Strings {}
