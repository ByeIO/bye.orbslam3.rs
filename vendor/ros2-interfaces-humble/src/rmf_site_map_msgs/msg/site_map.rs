use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SiteMap {
    pub encoding: u32,
    pub data: Vec<u8>,
}

impl SiteMap {
    pub const MAP_DATA_UNDEFINED: u32 = 0;
    pub const MAP_DATA_GPKG: u32 = 1;
    pub const MAP_DATA_GPKG_GZ: u32 = 2;
    pub const MAP_DATA_GEOJSON: u32 = 3;
    pub const MAP_DATA_GEOJSON_GZ: u32 = 4;
}

impl Default for SiteMap {
    fn default() -> Self {
        SiteMap {
            encoding: 0,
            data: Vec::new(),
        }
    }
}

impl ros2_client::Message for SiteMap {}
