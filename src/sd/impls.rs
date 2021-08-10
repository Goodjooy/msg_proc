use crate::sd::send_body::{NudgeSend};
use msg_chain::MessageChain;


use super::send_body::Kind;
use super::send_body::ReCall;
use super::{
    send_body::{SignleTagetSend, TempTagetSend},
    SendTarget,
};

pub fn new_firend_send<S>(
    target: &S,
    messages: Vec<Box<dyn MessageChain>>,
    quote: Option<i64>,
) -> SignleTagetSend
where
    S: SendTarget,
{
    SignleTagetSend {
        sessionKey: None,
        target: target.target_id(),
        quote,
        messageCahin: messages,
    }
}
pub fn new_group_send<S>(
    target: &S,
    messages: Vec<Box<dyn MessageChain>>,
    quote: Option<i64>,
) -> SignleTagetSend
where
    S: SendTarget,
{
    SignleTagetSend {
        sessionKey: None,
        target: target
            .target_group()
            .expect("Send Group Message Need Group Id"),
        quote,
        messageCahin: messages,
    }
}

pub fn new_temp_send<S: SendTarget>(
    target: &S,
    messages: Vec<Box<dyn MessageChain>>,
    quote: Option<i64>,
) -> TempTagetSend {
    TempTagetSend {
        sessionKey: None,
        target: target.target_id(),
        group: target
            .target_group()
            .expect("Send Temp Message Need Group Id"),
        quote: quote,
        messageCahin: messages,
    }
}



pub fn new_nudge_send<S: SendTarget>(target: &S, kind: Kind) -> NudgeSend {
    NudgeSend {
        sessionKey: None,
        target: target.target_id(),
        subject: target.target_group().unwrap_or(target.target_id()),
        kind,
    }
}

pub fn new_recall_send(target: i64) -> ReCall {
    ReCall {
        sessionKey: None,
        target,
    }
}
