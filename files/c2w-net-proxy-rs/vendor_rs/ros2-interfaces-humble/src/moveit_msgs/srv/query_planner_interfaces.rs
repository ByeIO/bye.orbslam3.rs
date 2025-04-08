use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QueryPlannerInterfacesRequest {

}

impl Default for QueryPlannerInterfacesRequest {
    fn default() -> Self {
        QueryPlannerInterfacesRequest {

        }
    }
}

impl ros2_client::Message for QueryPlannerInterfacesRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QueryPlannerInterfacesResponse {
    pub planner_interfaces: Vec<crate::moveit_msgs::msg::PlannerInterfaceDescription>,
}

impl Default for QueryPlannerInterfacesResponse {
    fn default() -> Self {
        QueryPlannerInterfacesResponse {
            planner_interfaces: Vec::new(),
        }
    }
}

impl ros2_client::Message for QueryPlannerInterfacesResponse {}


pub struct QueryPlannerInterfaces;
impl ros2_client::Service for QueryPlannerInterfaces {
    type Request = QueryPlannerInterfacesRequest;
    type Response = QueryPlannerInterfacesResponse;

    fn request_type_name(&self) -> &str { "QueryPlannerInterfacesRequest" }
    fn response_type_name(&self) -> &str { "QueryPlannerInterfacesResponse" }
}
