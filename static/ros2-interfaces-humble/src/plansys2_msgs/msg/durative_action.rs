use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DurativeAction {
    pub name: ::std::string::String,
    pub parameters: Vec<crate::plansys2_msgs::msg::Param>,
    pub at_start_requirements: crate::plansys2_msgs::msg::Tree,
    pub over_all_requirements: crate::plansys2_msgs::msg::Tree,
    pub at_end_requirements: crate::plansys2_msgs::msg::Tree,
    pub at_start_effects: crate::plansys2_msgs::msg::Tree,
    pub at_end_effects: crate::plansys2_msgs::msg::Tree,
}

impl Default for DurativeAction {
    fn default() -> Self {
        DurativeAction {
            name: ::std::string::String::new(),
            parameters: Vec::new(),
            at_start_requirements: crate::plansys2_msgs::msg::Tree::default(),
            over_all_requirements: crate::plansys2_msgs::msg::Tree::default(),
            at_end_requirements: crate::plansys2_msgs::msg::Tree::default(),
            at_start_effects: crate::plansys2_msgs::msg::Tree::default(),
            at_end_effects: crate::plansys2_msgs::msg::Tree::default(),
        }
    }
}

impl ros2_client::Message for DurativeAction {}
