use std::collections::HashMap;

use crate::{Permission, Platform};
use from_map::FromMap;
use serde_json::Value;

use msg_chain::IntoChainMeta;
mod impls;

pub trait FromMap: Sized {
    fn from_map(map: &HashMap<String, Value>) -> Option<Self>;
}

#[derive(serde::Deserialize, FromMap)]
struct Group {
    id: u64,
    name: String,
    permission: Permission,
}
#[derive(serde::Deserialize, FromMap)]
pub struct GroupBaseSender {
    id: u64,
    member_name: String,
    special_title: String,
    permission: Permission,
    join_timestamp: u64,
    last_dpeak_timestamp: u64,
    mute_time_remaining: u64,
    group: Group,
}
#[derive(serde::Deserialize, FromMap)]
pub struct SingleSender {
    id: u64,
    nickname: String,
    remark: String,
}
#[derive(serde::Deserialize, FromMap)]
pub struct ClientSender {
    id: u64,
    platform: Platform,
}
