use crate::rev::sender::ClientSender;
use crate::rev::sender::FromMap;
use crate::rev::sender::GroupBaseSender;
use crate::rev::sender::SingleSender;
use crate::{generate_sender_picker, rev::msg_chain::message_chain_loader};
use ::msg_chain::{ChainMeta, IntoChainMeta};
use serde_json::Map;
use serde_json::Value;
use std::collections::btree_map::VacantEntry;
use std::collections::HashMap;

use crate::MessageRev;

mod msg_chain;
mod sender;

generate_sender_picker!(GroupBaseSender, SingleSender, ClientSender);

#[derive(serde::Serialize, serde::Deserialize)]
pub struct MessageRevMid {
    r#type: String,
    sender: HashMap<String, Value>,
    messageChain: Vec<HashMap<String, Value>>,
}

fn into_hashmap(map: Map<String, Value>) -> HashMap<String, Value> {
    let temp = map.into_iter().collect();

    temp
}

impl MessageRevMid {
    pub fn into_message(self) -> Option<MessageRev> {
        let ty = self.r#type.clone();
        let sender = sender_picker(&self.sender)?;
        let messages = self
            .messageChain
            .iter()
            .map(|map| {
                map.into_iter()
                    .map(|f| ((f.0.clone(), f.1.into_chain())))
                    .collect::<HashMap<String, ChainMeta>>()
            })
            .map(|f| message_chain_loader(&f))
            .filter(|f| if let None = f { false } else { true })
            .map(|f| f.unwrap())
            .collect();

        let msg = MessageRev {
            msg_type: ty,
            sender: sender,
            chain: messages,
        };

        Some(msg)
    }
}

impl FromMap for MessageRevMid {
    fn from_map(map: &HashMap<String, Value>) -> Option<Self> {
        let ty = map.get("type").and_then(|f| f.as_str())?.to_string();
        let sender = map.get("Sneder").and_then(|f| f.as_object())?.clone();
        let sender = into_hashmap(sender);
        let msg_cahin = map
            .get("messageChain")
            .and_then(|v| v.as_array())?
            .clone()
            .iter()
            .map(|f| f.as_object())
            .filter(|f| if let None = f { false } else { true })
            .map(|f| f.unwrap())
            .map(|f| into_hashmap(f.clone()))
            .collect();

        Some(Self {
            r#type: ty,
            messageChain: msg_cahin,
            sender,
        })
    }
}
