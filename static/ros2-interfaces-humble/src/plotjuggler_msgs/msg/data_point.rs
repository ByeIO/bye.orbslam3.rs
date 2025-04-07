use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataPoint {
    pub name_index: u16,
    pub stamp: f64,
    pub value: f64,
}

impl Default for DataPoint {
    fn default() -> Self {
        DataPoint {
            name_index: 0,
            stamp: 0.0,
            value: 0.0,
        }
    }
}

impl ros2_client::Message for DataPoint {}
