use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnsToCmdConnect {
    pub header: crate::std_msgs::msg::Header,
    pub command: u8,
    pub cmd_connect_ans_d7: u8,
    pub cmd_connect_ans_d6: u8,
    pub cmd_connect_ans_d5: u8,
    pub cmd_connect_ans_d4: u8,
    pub cmd_connect_ans_d3: u8,
    pub cmd_connect_ans_d2: u8,
    pub cmd_connect_ans_d1: u8,
}

impl Default for AnsToCmdConnect {
    fn default() -> Self {
        AnsToCmdConnect {
            header: crate::std_msgs::msg::Header::default(),
            command: 0,
            cmd_connect_ans_d7: 0,
            cmd_connect_ans_d6: 0,
            cmd_connect_ans_d5: 0,
            cmd_connect_ans_d4: 0,
            cmd_connect_ans_d3: 0,
            cmd_connect_ans_d2: 0,
            cmd_connect_ans_d1: 0,
        }
    }
}

impl ros2_client::Message for AnsToCmdConnect {}
