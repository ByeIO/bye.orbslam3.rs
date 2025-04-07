use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SensorModulationPerformance {
    pub detection_of_measurement_program: u8,
    pub modulation_id: u8,
    pub distance_range_scaling: f32,
    pub separability_distance: f32,
    pub separability_relative_velocity: f32,
    pub precision_distance: f32,
    pub precision_relative_velocity: f32,
    pub covariance_of_distance_and_relative_velocity: f32,
    pub minimum_measurable_distance: f32,
    pub maximum_measurable_distance: f32,
    pub minimum_measurable_relative_velocity: f32,
    pub maximum_measurable_relative_velocity: f32,
}

impl Default for SensorModulationPerformance {
    fn default() -> Self {
        SensorModulationPerformance {
            detection_of_measurement_program: 0,
            modulation_id: 0,
            distance_range_scaling: 0.0,
            separability_distance: 0.0,
            separability_relative_velocity: 0.0,
            precision_distance: 0.0,
            precision_relative_velocity: 0.0,
            covariance_of_distance_and_relative_velocity: 0.0,
            minimum_measurable_distance: 0.0,
            maximum_measurable_distance: 0.0,
            minimum_measurable_relative_velocity: 0.0,
            maximum_measurable_relative_velocity: 0.0,
        }
    }
}

impl ros2_client::Message for SensorModulationPerformance {}
