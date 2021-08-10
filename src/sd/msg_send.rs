use std::{collections::HashMap, fmt::Display};

use serde::{ser::SerializeMap, Serialize};
use serde_json::Value;

use super::send_body::{NudgeSend, ReCall, SignleTagetSend, TempTagetSend};

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

pub struct SendBody(HashMap<String, Value>);

impl SendBody {
    pub fn set_session_key<T: Display>(&mut self, key: &T) {
        self.0
            .insert("sessionKey".to_string(), Value::String(key.to_string()));
    }
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
