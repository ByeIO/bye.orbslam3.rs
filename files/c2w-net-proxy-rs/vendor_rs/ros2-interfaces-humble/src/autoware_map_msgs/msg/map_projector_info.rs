use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MapProjectorInfo {
    pub projector_type: ::std::string::String,
    pub vertical_datum: ::std::string::String,
    pub mgrs_grid: ::std::string::String,
    pub map_origin: crate::geographic_msgs::msg::GeoPoint,
}

impl MapProjectorInfo {
    pub const LOCAL: &'static str = "Local";
    pub const LOCAL_CARTESIAN_UTM: &'static str = "LocalCartesianUTM";
    pub const MGRS: &'static str = "MGRS";
    pub const TRANSVERSE_MERCATOR: &'static str = "TransverseMercator";
    pub const WGS84: &'static str = "WGS84";
    pub const EGM2008: &'static str = "EGM2008";
}

impl Default for MapProjectorInfo {
    fn default() -> Self {
        MapProjectorInfo {
            projector_type: ::std::string::String::new(),
            vertical_datum: ::std::string::String::new(),
            mgrs_grid: ::std::string::String::new(),
            map_origin: crate::geographic_msgs::msg::GeoPoint::default(),
        }
    }
}

impl ros2_client::Message for MapProjectorInfo {}
