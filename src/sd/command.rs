use std::fmt::Display;

use msg_chain::MessageChain;
use serde_json::{Map, Value};

use super::msg_send::SendBody;
#[derive(Debug)]
pub struct SendCommand {
    pub main_cmd: String,
    pub side_cmd: Option<String>,
}

impl SendCommand {
    pub fn get_cmd(self) -> (String, Option<String>) {
        (self.main_cmd, self.side_cmd)
    }
}
#[derive(Debug)]
pub struct CmdWithSendBody {
    pub cmd: SendCommand,
    pub body: SendBody,
}

impl CmdWithSendBody {
    pub fn set_session_key<T: Display>(&mut self, key: &T) {
        self.body.set_session_key(key)
    }

    pub fn get_send_chain(&self) -> Vec<Box<dyn MessageChain>> {
        self.body.get_send_chain()
    }
}
