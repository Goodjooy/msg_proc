use msg_chain::{ChainMeta, FromChainMeta};

use super::{ClientSender, Group, GroupBaseSender, SingleSender};
use crate::{Permission, Platform, Sender};

impl Sender for GroupBaseSender {
    fn get_sender_permission(&self) -> Option<&crate::Permission> {
        Some(&self.permission)
    }

    fn get_sender_name(&self) -> Option<&String> {
        Some(&self.member_name)
    }

    fn get_sender_market(&self) -> Option<&String> {
        Some(&self.special_title)
    }

    fn get_group_from(&self) -> Option<&u64> {
        Some(&self.group.id)
    }

    fn get_self_permession(&self) -> Option<&crate::Permission> {
        Some(&self.group.permission)
    }

    fn get_sender_id(&self) -> &u64 {
        &self.id
    }
    fn get_group_name(&self) -> Option<&String> {
        Some(&self.group.name)
    }
}

impl Sender for SingleSender {
    fn get_sender_id(&self) -> &u64 {
        &self.id
    }

    fn get_sender_name(&self) -> Option<&String> {
        Some(&self.nickname)
    }

    fn get_sender_market(&self) -> Option<&String> {
        Some(&self.remark)
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

impl Sender for ClientSender {
    fn get_sender_id(&self) -> &u64 {
        &self.id
    }
    fn get_platform(&self) -> Option<&Platform> {
        Some(&self.platform)
    }
}

impl FromChainMeta for Group{
    fn from_chain(chain: Option<&msg_chain::ChainMeta>) -> Option<Self> {
        if let ChainMeta::MapOwn(map)=chain?{
            let id=u64::from_chain(map.get("id"))?;
            let name=String::from_chain(map.get("name"))?;
            let permission_s=String::from_chain(map.get("permission"))?;
            let permission=Permission::from_str(&permission_s)?;

            Some(Self{
                id,name,permission
            })
        }else{None}
    }
}
#[macro_export]
macro_rules! generate_sender_picker {
    ($($t:ty),*) => {
        pub fn sender_picker(map:&HashMap<String,serde_json::Value>)->Option<Box<dyn crate::Sender>>{
            $(
                if let Some(value)=<$t>::from_map(map){
                    return Some(Box::new(value));
                }
            )*
            None
        }
    };
}