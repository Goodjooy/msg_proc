use crate::sd::send_body::NudgeSend;
use msg_chain::MessageChain;

use super::command::CmdWithSendBody;
use super::msg_send::MsgSend;
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
) -> CmdWithSendBody
where
    S: SendTarget,
{
    SignleTagetSend {
        sessionKey: None,
        target: target.target_id(),
        quote,
        messageCahin: messages,
    }
    .into_map()
    .into_cmd_send_body("sendFriendMessage", None)
}
pub fn new_group_send<S>(
    target: &S,
    messages: Vec<Box<dyn MessageChain>>,
    quote: Option<i64>,
) -> CmdWithSendBody
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
    .into_map()
    .into_cmd_send_body("sendGroupMessage", None)
}

pub fn new_temp_send<S: SendTarget>(
    target: &S,
    messages: Vec<Box<dyn MessageChain>>,
    quote: Option<i64>,
) -> CmdWithSendBody {
    TempTagetSend {
        sessionKey: None,
        target: target.target_id(),
        group: target
            .target_group()
            .expect("Send Temp Message Need Group Id"),
        quote: quote,
        messageCahin: messages,
    }
    .into_map()
    .into_cmd_send_body("sendTempMessage", None)
}

pub fn new_source_send<S: SendTarget>(
    msg_ty: &str,
    target: &S,
    messages: Vec<Box<dyn MessageChain>>,
    quote: Option<i64>,
) -> Option<CmdWithSendBody> {
    match msg_ty {
        "FriendMessage" => Some(new_firend_send(target, messages, quote)),
        "GroupMessage" => Some(new_group_send(target, messages, quote)),
        "TempMessage" => Some(new_temp_send(target, messages, quote)),
        _ => None,
    }
}

pub fn new_nudge_send<S: SendTarget>(target: &S, kind: Kind) -> CmdWithSendBody {
    NudgeSend {
        sessionKey: None,
        target: target.target_id(),
        subject: target.target_group().unwrap_or(target.target_id()),
        kind,
    }
    .into_map()
    .into_cmd_send_body("sendNudge", None)
}

pub fn new_recall_send(target: i64) -> CmdWithSendBody {
    ReCall {
        sessionKey: None,
        target,
    }
    .into_map()
    .into_cmd_send_body("recall", None)
}
