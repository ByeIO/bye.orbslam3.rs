use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatisticDataType {

}

impl StatisticDataType {
    pub const STATISTICS_DATA_TYPE_UNINITIALIZED: u8 = 0;
    pub const STATISTICS_DATA_TYPE_AVERAGE: u8 = 1;
    pub const STATISTICS_DATA_TYPE_MINIMUM: u8 = 2;
    pub const STATISTICS_DATA_TYPE_MAXIMUM: u8 = 3;
    pub const STATISTICS_DATA_TYPE_STDDEV: u8 = 4;
    pub const STATISTICS_DATA_TYPE_SAMPLE_COUNT: u8 = 5;
}

impl Default for StatisticDataType {
    fn default() -> Self {
        StatisticDataType {

        }
    }
}

impl ros2_client::Message for StatisticDataType {}
