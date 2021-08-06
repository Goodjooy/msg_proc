use msg_chain::MessageChain;

use crate::rev::msg_chain::{At, Image, Plain};

use super::BulidTarget;

pub struct ChainHandle(BulidTarget);

pub trait ToMsgHandle {
    fn to_msg_handle(self) -> ChainHandle;
}

impl ToMsgHandle for BulidTarget {
    fn to_msg_handle(self) -> ChainHandle {
        ChainHandle::new(self)
    }
}

impl ChainHandle {
    pub fn new(src: BulidTarget) -> Self {
        Self(src)
    }
}

impl ChainHandle {
    pub fn get_img(&self, index: usize) -> Option<&Box<dyn MessageChain>> {
        let res = self
            .0
            .iter()
            .filter(|item| item.get_type().to_lowercase() == "image")
            .skip(index)
            .find(|_| true);

        res
    }

    pub fn all_image(&self)->Vec<Image>{
         self.0
        .iter()
        .filter(|item|item.get_type().to_lowercase()=="image")
        .map(|img|img.into_target::<Image>().unwrap())
        .collect()
    }

    pub fn conbin_plain(&self) -> Option<String> {
        let res = self
            .0
            .iter()
            .filter(|item| item.get_type().to_lowercase() == "plain")
            .map(|s| s.into_target::<Plain>().unwrap())
            .map(|pla| pla.text)
            .reduce(|s1, s2| format!("{}{}", s1, s2));

        res
    }

    pub fn is_at_target(&self, target: u64) -> bool {
        let res = self
            .0
            .iter()
            .filter(|item| item.get_type().to_lowercase() == "at")
            .map(|at| at.into_target::<At>().unwrap())
            .any(|f| f.target == target);
        res
    }

    
}
