use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlannerInterfaceDescription {
    pub name: ::std::string::String,
    pub pipeline_id: ::std::string::String,
    pub planner_ids: Vec<::std::string::String>,
}

impl Default for PlannerInterfaceDescription {
    fn default() -> Self {
        PlannerInterfaceDescription {
            name: ::std::string::String::new(),
            pipeline_id: ::std::string::String::new(),
            planner_ids: Vec::new(),
        }
    }
}

impl ros2_client::Message for PlannerInterfaceDescription {}
