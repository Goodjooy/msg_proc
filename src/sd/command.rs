use std::fmt::Display;

use super::msg_send::SendBody;

pub struct SendCommand {
    pub main_cmd: String,
    pub side_cmd: Option<String>,
}

impl SendCommand {
    pub fn get_cmd(self) -> (String, Option<String>) {
        (self.main_cmd, self.side_cmd)
    }
}

pub struct CmdWithSendBody {
    pub cmd: SendCommand,
    pub body: SendBody,
}

impl CmdWithSendBody {
    pub fn set_session_key<T: Display>(&mut self, key: &T) {
        self.body.set_session_key(key)
    }
}
