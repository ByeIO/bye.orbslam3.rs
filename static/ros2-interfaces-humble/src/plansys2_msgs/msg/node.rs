use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Node {
    pub node_type: u8,
    pub expression_type: u8,
    pub modifier_type: u8,
    pub node_id: u32,
    pub children: Vec<u32>,
    pub name: ::std::string::String,
    pub parameters: Vec<crate::plansys2_msgs::msg::Param>,
    pub value: f64,
    pub negate: bool,
}

impl Node {
    pub const UNKNOWN: u8 = 0;
    pub const AND: u8 = 1;
    pub const OR: u8 = 2;
    pub const NOT: u8 = 3;
    pub const ACTION: u8 = 4;
    pub const PREDICATE: u8 = 5;
    pub const FUNCTION: u8 = 6;
    pub const EXPRESSION: u8 = 7;
    pub const FUNCTION_MODIFIER: u8 = 8;
    pub const NUMBER: u8 = 9;
    pub const COMP_EQ: u8 = 10;
    pub const COMP_GE: u8 = 11;
    pub const COMP_GT: u8 = 12;
    pub const COMP_LE: u8 = 13;
    pub const COMP_LT: u8 = 14;
    pub const ARITH_MULT: u8 = 15;
    pub const ARITH_DIV: u8 = 16;
    pub const ARITH_ADD: u8 = 17;
    pub const ARITH_SUB: u8 = 18;
    pub const ASSIGN: u8 = 19;
    pub const INCREASE: u8 = 20;
    pub const DECREASE: u8 = 21;
    pub const SCALE_UP: u8 = 22;
    pub const SCALE_DOWN: u8 = 23;
}

impl Default for Node {
    fn default() -> Self {
        Node {
            node_type: 0,
            expression_type: 0,
            modifier_type: 0,
            node_id: 0,
            children: Vec::new(),
            name: ::std::string::String::new(),
            parameters: Vec::new(),
            value: 0.0,
            negate: false,
        }
    }
}

impl ros2_client::Message for Node {}
