use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Plan {
    pub items: Vec<crate::plansys2_msgs::msg::PlanItem>,
}

impl Default for Plan {
    fn default() -> Self {
        Plan {
            items: Vec::new(),
        }
    }
}

impl ros2_client::Message for Plan {}
