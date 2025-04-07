use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EcuInfo {
    pub header: crate::std_msgs::msg::Header,
    pub name: ::std::string::String,
    pub version: ::std::string::String,
    pub mac_addr: ::std::string::String,
    pub config_hash: ::std::string::String,
    pub config_count_modified: u8,
    pub config_count_configured: u8,
    pub config_nvm_blank: bool,
    pub config_nvm_write_pending: bool,
    pub build_date: ::std::string::String,
    pub license_date: ::std::string::String,
    pub control_licensed: bool,
    pub log_filename: ::std::string::String,
    pub log_filesystem_present: bool,
    pub log_fault: bool,
}

impl Default for EcuInfo {
    fn default() -> Self {
        EcuInfo {
            header: crate::std_msgs::msg::Header::default(),
            name: ::std::string::String::new(),
            version: ::std::string::String::new(),
            mac_addr: ::std::string::String::new(),
            config_hash: ::std::string::String::new(),
            config_count_modified: 0,
            config_count_configured: 0,
            config_nvm_blank: false,
            config_nvm_write_pending: false,
            build_date: ::std::string::String::new(),
            license_date: ::std::string::String::new(),
            control_licensed: false,
            log_filename: ::std::string::String::new(),
            log_filesystem_present: false,
            log_fault: false,
        }
    }
}

impl ros2_client::Message for EcuInfo {}
