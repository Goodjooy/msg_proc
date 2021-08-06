use serde::Serialize;

use msg_chain::MessageChain;

pub trait MsgSend: Sized {
    fn into_json(self) -> serde_json::Value
    where
        Self: Serialize,
    {
        serde_json::to_value(self).expect("Failure into Json")
    }
}

impl MsgSend for SignleTagetSend {}
#[derive(serde::Serialize)]
pub struct SignleTagetSend {
    pub sessionKey: Option<String>,
    pub target: u64,
    pub quote: Option<i64>,
    pub messageCahin: Vec<Box<dyn MessageChain>>,
}

impl MsgSend for TempTagetSend {}
#[derive(serde::Serialize)]
pub struct TempTagetSend {
    pub sessionKey: Option<String>,
    pub target: u64,
    pub group: u64,
    pub quote: Option<i64>,
    pub messageCahin: Vec<Box<dyn MessageChain>>,
}
impl MsgSend for NudgeSend {}
#[derive(serde::Serialize)]
pub struct NudgeSend {
    pub sessionKey: Option<String>,
    pub target: u64,
    pub subject: u64,
    pub kind: Kind,
}
impl MsgSend for ReCall {}
#[derive(serde::Serialize)]
pub struct ReCall {
    pub sessionKey: Option<String>,
    pub target: i64,
}

#[derive(serde::Serialize)]
pub enum Kind {
    Firend,
    Group,
    Stranger,
}
