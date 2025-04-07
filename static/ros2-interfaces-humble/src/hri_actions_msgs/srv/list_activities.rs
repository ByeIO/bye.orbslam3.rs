use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListActivitiesRequest {

}

impl Default for ListActivitiesRequest {
    fn default() -> Self {
        ListActivitiesRequest {

        }
    }
}

impl ros2_client::Message for ListActivitiesRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListActivitiesResponse {
    pub activities: crate::hri_actions_msgs::msg::ActivityList,
}

impl Default for ListActivitiesResponse {
    fn default() -> Self {
        ListActivitiesResponse {
            activities: crate::hri_actions_msgs::msg::ActivityList::default(),
        }
    }
}

impl ros2_client::Message for ListActivitiesResponse {}


pub struct ListActivities;
impl ros2_client::Service for ListActivities {
    type Request = ListActivitiesRequest;
    type Response = ListActivitiesResponse;

    fn request_type_name(&self) -> &str { "ListActivitiesRequest" }
    fn response_type_name(&self) -> &str { "ListActivitiesResponse" }
}
