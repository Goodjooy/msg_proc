use msg_chain::MessageChain;

mod impls;
mod rev;
mod sd;

pub trait Sender:Send {
    fn get_sender_id(&self) -> &u64;
    fn get_sender_permission(&self) -> Option<&Permission> {
        None
    }
    fn get_sender_name(&self) -> Option<&String> {
        None
    }
    fn get_sender_market(&self) -> Option<&String> {
        None
    }

    fn get_group_from(&self) -> Option<&u64> {
        None
    }
    fn get_group_name(&self) -> Option<&String> {
        None
    }

    fn get_self_permession(&self) -> Option<&Permission> {
        None
    }

    fn get_platform(&self) -> Option<&Platform> {
        None
    }
}

pub struct MessageRev {
    pub msg_type: String,
    pub sender: Box<dyn Sender>,
    pub chain: Vec<Box<dyn MessageChain>>,
}



pub enum Permission {
    Owner,
    Member,
    Admin,
}
pub enum Platform {
    Mobile,
}

pub mod recive {
    use self::sender::FromMap;
    use crate::rev::MessageRevMid;
    use serde_json::Value;
    use std::collections::HashMap;

    pub use super::Sender;
    pub use crate::MessageRev;

    pub mod sender {
        pub use crate::rev::sender::FromMap;
        /// senders
        pub use crate::rev::sender::{ClientSender, GroupBaseSender, SingleSender};
        /// load func
        pub use crate::rev::sender_picker;
    }

    pub fn load_recive_data(map: &HashMap<String, Value>) -> Option<MessageRev> {
        let msg_mid = MessageRevMid::from_map(map)?;
        msg_mid.into_message()
    }
}

pub mod chain {
    /// msggsage chain
    pub use crate::rev::msg_chain;
    /// load func
    pub use crate::rev::msg_chain::message_chain_loader;

    pub use crate::sd::utils::chain_builder;
    pub use crate::sd::utils::chain_handle;
}

pub mod send {
    pub use crate::sd::utils;
    pub mod target {
        /// custom trait
        pub use crate::sd::CustomTarget;
        /// send target trait
        pub use crate::sd::SendTarget;
        /// custom grenerate trait
        pub use crate::target_generate;
    }
    pub mod cmd{
        pub use crate::sd::command::{CmdWithSendBody,SendCommand};
    }
    pub mod body {
        pub use crate::sd::msg_send::MsgSend;
        pub use crate::sd::msg_send::SendBody;
        pub use crate::sd::send_body::{NudgeSend, ReCall, SignleTagetSend, TempTagetSend};
    }
    pub mod contain {
        pub use crate::sd::impls::{
            new_firend_send, new_group_send, new_nudge_send, new_recall_send, new_source_send,
            new_temp_send
        };
    }

    pub use crate::sd::impls::SendBodyTypeNotFoundError;
}
