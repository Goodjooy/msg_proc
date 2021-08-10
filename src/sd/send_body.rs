use msg_chain::MessageChain;

#[derive(serde::Serialize)]
pub struct SignleTagetSend {
    pub sessionKey: Option<String>,
    pub target: u64,
    pub quote: Option<i64>,
    pub messageCahin: Vec<Box<dyn MessageChain>>,
}

#[derive(serde::Serialize)]
pub struct TempTagetSend {
    pub sessionKey: Option<String>,
    pub target: u64,
    pub group: u64,
    pub quote: Option<i64>,
    pub messageCahin: Vec<Box<dyn MessageChain>>,
}

#[derive(serde::Serialize)]
pub struct NudgeSend {
    pub sessionKey: Option<String>,
    pub target: u64,
    pub subject: u64,
    pub kind: Kind,
}

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
