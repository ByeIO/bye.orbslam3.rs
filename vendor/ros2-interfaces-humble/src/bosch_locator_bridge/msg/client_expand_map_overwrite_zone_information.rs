use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClientExpandMapOverwriteZoneInformation {
    pub id: i64,
    pub name: ::std::string::String,
    #[serde(rename = "type")]    pub type_: i64,
    pub polygon: Vec<crate::geometry_msgs::msg::Point32>,
}

impl ClientExpandMapOverwriteZoneInformation {
    pub const TYPE_ENFORCE: i32 = 1;
    pub const TYPE_IGNORE: i32 = 2;
    pub const TYPE_DELETE: i32 = 3;
}

impl Default for ClientExpandMapOverwriteZoneInformation {
    fn default() -> Self {
        ClientExpandMapOverwriteZoneInformation {
            id: 0,
            name: ::std::string::String::new(),
            type_: 0,
            polygon: Vec::new(),
        }
    }
}

impl ros2_client::Message for ClientExpandMapOverwriteZoneInformation {}
