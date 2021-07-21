use crate::rev::msg_chain::message_chain_loader;
use crate::rev::sender::sender_picker;
use std::collections::HashMap;

use ::msg_chain::{ChainMeta, IntoChainMeta};
use serde_json::Value;

use crate::MessageRev;

mod msg_chain;
mod sender;

#[derive(serde::Serialize,serde::Deserialize)]
pub struct MessageRevMid{
    r#type:String,
    sender:HashMap<String,Value>,
    messageChain:Vec<HashMap<String,Value>>
}

impl MessageRevMid {
    pub fn into_message(self)->Option<MessageRev>{
        let ty=self.r#type.clone();
        let sender=sender_picker(&self.sender)?;
        let messages=self.messageChain
        .iter()
        .map(|map|{
            map
            .into_iter()
            .map(|f|((f.0.clone(),f.1.into_chain())))
            .collect::<HashMap<String,ChainMeta>>()
        })
        .map(|f|message_chain_loader(&f))
        .filter(|f| if let None=f{false}else{true})
        .map(|f|f.unwrap())
        .collect()
        ;
        
        let msg=MessageRev{
            msg_type:ty,
            sender:sender,
            chain:messages
        };

        Some(msg)
    }
}