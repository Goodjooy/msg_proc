use crate::chain::message_chain_loader;
use std::{collections::HashMap, fmt::Display};

use msg_chain::{IntoChainMeta, MessageChain};
use serde::{ser::SerializeMap, Serialize};
use serde_json::{Map, Value};

use super::{
    command::{CmdWithSendBody, SendCommand},
    send_body::{NudgeSend, ReCall, SignleTagetSend, TempTagetSend},
};

pub trait MsgSend: Sized {
    fn into_json(self) -> serde_json::Value
    where
        Self: Serialize,
    {
        let v = serde_json::to_value(self).expect("Failure into Json");
        v
    }

    fn into_map(self) -> SendBody
    where
        Self: Serialize,
    {
        match self.into_json() {
            serde_json::Value::Object(obj) => SendBody(obj.into_iter().collect()),
            _ => panic!("Not A Object"),
        }
    }
}

impl MsgSend for SignleTagetSend {}
impl MsgSend for TempTagetSend {}
impl MsgSend for NudgeSend {}
impl MsgSend for ReCall {}

#[derive(Debug)]
pub struct SendBody(HashMap<String, Value>);

impl SendBody {
    pub fn set_session_key<T: Display>(&mut self, key: &T) {
        self.0
            .insert("sessionKey".to_string(), Value::String(key.to_string()));
    }

    pub fn into_cmd_send_body(self, cmd: &str, side_cmd: Option<&str>) -> CmdWithSendBody {
        let cmd = SendCommand {
            main_cmd: cmd.to_string(),
            side_cmd: side_cmd.and_then(|s| Some(s.to_string())),
        };
        CmdWithSendBody { cmd, body: self }
    }
    pub fn get_send_chain(&self) -> Vec<Box<dyn MessageChain>> {
        let res = self
            .0
            .get("messageCahin")
            .expect("Target SendBody Do not have MessageChain");

        if let Value::Array(arr) = res {
            into_msg_chain(arr)
        } else {
            panic!("Message Chain Not A Object");
        }
    }
}

fn into_msg_chain(chain: &Vec<Value>) -> Vec<Box<dyn MessageChain>> {
    chain
        .iter()
        .map(|f| {
            if let Value::Object(obj) = f {
                let map = obj
                    .into_iter()
                    .map(|(k, v)| (k.clone(), v.into_chain()))
                    .collect();
                message_chain_loader(&map)
                    .expect(&format!("Can not Load As Message Chain | {:?}", f))
            } else {
                panic!("Message Chain Not A Json Object | {:?}", f);
            }
        })
        .collect::<Vec<_>>()
}

impl Serialize for SendBody {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(None)?;

        for (k, v) in &self.0 {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}
