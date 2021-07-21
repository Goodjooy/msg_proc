use msg_chain::msg_loader_generate;
use msg_chain::{ChainMeta, FromChainMeta, IntoChainMeta, LoadFormMap, MessageChain};
use std::collections::HashMap;

msg_loader_generate!(
    Source, Plain, At, AtAll, Face, Image, FlashImage, Voice, Xml, App, Poke, Dice, MusicShare,
    Forward, File
);

#[derive(MessageChain, LoadFormMap)]
pub struct Source {
    pub id: i64,
    pub time: u64,
}

#[derive(MessageChain, LoadFormMap)]
pub struct Quote {
    pub id: u64,
    pub group_id: u64,
    pub sender_id: u64,
    pub target_id: u64,
    pub origin: Vec<HashMap<&'static str, ChainMeta>>,
}

#[derive(MessageChain, LoadFormMap)]
pub struct Plain {
    pub text: String,
}
#[derive(MessageChain, LoadFormMap)]
pub struct At {
    pub target: u64,
    pub display: String,
}
#[derive(MessageChain, LoadFormMap)]
pub struct AtAll;
#[derive(MessageChain, LoadFormMap)]
pub struct Face {
    pub face_id: i32,
    pub name: String,
}
#[derive(MessageChain, LoadFormMap)]
pub struct Image {
    pub image_id: Option<String>,
    pub url: Option<String>,
    pub path: Option<String>,
    pub base64: Option<String>,
}

#[derive(MessageChain, LoadFormMap)]
pub struct FlashImage {
    pub image_id: Option<String>,
    pub url: Option<String>,
    pub path: Option<String>,
    pub base64: Option<String>,
}

#[derive(MessageChain, LoadFormMap)]
pub struct Voice {
    pub voice_id: Option<String>,
    pub url: Option<String>,
    pub path: Option<String>,
    pub base64: Option<String>,
}
#[derive(MessageChain, LoadFormMap)]
struct Xml {
    pub xml: String,
}
#[derive(MessageChain, LoadFormMap)]
struct Json {
    pub json: String,
}
#[derive(MessageChain, LoadFormMap)]
pub struct App {
    pub content: String,
}

pub enum PokeName {
    Poke,        // 戳一戳
    ShowLove,    // 比心
    Like,        // 点赞
    Heartbroken, // 心碎
    SixSixSix,   // 666
    FangDaZhao,  // 放大招
}
impl IntoChainMeta for PokeName {
    fn into_chain(&self) -> ChainMeta {
        let v = match self {
            PokeName::Poke => "Poke",
            PokeName::ShowLove => "ShowLove",
            PokeName::Like => "Like",
            PokeName::Heartbroken => "Heartbroken",
            PokeName::SixSixSix => "SixSixSix",
            PokeName::FangDaZhao => "FangDaZhao",
        };
        v.into_chain()
    }
}

impl FromChainMeta for PokeName {
    fn from_chain(chain: Option<&ChainMeta>) -> Option<Self> {
        match chain? {
            ChainMeta::Str(s) => match &s[..] {
                "Poke" => Some(PokeName::Poke),               //戳一戳
                "ShowLove" => Some(PokeName::ShowLove),       //比心
                "Like" => Some(PokeName::Like),               //点赞
                "Heartbroken" => Some(PokeName::Heartbroken), //心碎
                "SixSixSix" => Some(PokeName::SixSixSix),     //666
                "FangDaZhao" => Some(PokeName::FangDaZhao),   //放大招
                _ => None,
            },
            _ => None,
        }
    }
}
#[derive(MessageChain, LoadFormMap)]
pub struct Poke {
    pub name: PokeName,
}
#[derive(MessageChain, LoadFormMap)]
pub struct Dice {
    pub value: i32,
}
#[derive(MessageChain, LoadFormMap)]
pub struct MusicShare {
    pub kind: String,       //	类型
    pub title: String,      //	标题
    pub summary: String,    //	概括
    pub jump_url: String,    //	跳转路径
    pub picture_url: String, //	封面路径
    pub music_url: String,   //	音源路径
    pub brief: String,      //	简介
}
#[derive(MessageChain, LoadFormMap)]
pub struct Forward {
    pub node_list: Vec<HashMap<&'static str, ChainMeta>>,
}
#[derive(MessageChain, LoadFormMap)]
pub struct File {
    pub id: String,
    pub name: String,
    pub size: u64,
}