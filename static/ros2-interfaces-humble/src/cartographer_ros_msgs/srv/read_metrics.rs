use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReadMetricsRequest {

}

impl Default for ReadMetricsRequest {
    fn default() -> Self {
        ReadMetricsRequest {

        }
    }
}

impl ros2_client::Message for ReadMetricsRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReadMetricsResponse {
    pub status: crate::cartographer_ros_msgs::msg::StatusResponse,
    pub metric_families: Vec<crate::cartographer_ros_msgs::msg::MetricFamily>,
    pub timestamp: crate::builtin_interfaces::msg::Time,
}

impl Default for ReadMetricsResponse {
    fn default() -> Self {
        ReadMetricsResponse {
            status: crate::cartographer_ros_msgs::msg::StatusResponse::default(),
            metric_families: Vec::new(),
            timestamp: crate::builtin_interfaces::msg::Time::default(),
        }
    }
}

impl ros2_client::Message for ReadMetricsResponse {}


pub struct ReadMetrics;
impl ros2_client::Service for ReadMetrics {
    type Request = ReadMetricsRequest;
    type Response = ReadMetricsResponse;

    fn request_type_name(&self) -> &str { "ReadMetricsRequest" }
    fn response_type_name(&self) -> &str { "ReadMetricsResponse" }
}
