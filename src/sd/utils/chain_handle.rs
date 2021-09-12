use msg_chain::MessageChain;

use crate::rev::msg_chain::{At, Image, Plain};

use super::BulidTarget;

pub struct ChainHandle<'s>(&'s BulidTarget);

pub trait ToMsgHandle<'s> {
    fn to_msg_handle(&self) -> ChainHandle;
}

impl<'s> ToMsgHandle<'s> for BulidTarget {
    fn to_msg_handle(&self) -> ChainHandle {
        ChainHandle::new(self)
    }
}

impl<'s> ChainHandle<'s> {
    pub fn new(src: &'s BulidTarget) -> Self {
        Self(src)
    }
}

impl ChainHandle<'_> {
    pub fn get_img(&self, index: usize) -> Option<&Box<dyn MessageChain>> {
        let res = self
            .0
            .iter()
            .filter(|item| item.get_type().to_lowercase() == "image")
            .skip(index)
            .find(|_| true);

        res
    }

    pub fn all_image(&self) -> Vec<(usize, Image)> {
        self.0
            .iter()
            .enumerate()
            .filter(|item| item.1.get_type().to_lowercase() == "image")
            .map(|(index, img)| (index, img.into_target::<Image>().unwrap()))
            .collect()
    }
    pub fn replace_image(&mut self,imgs:Vec<(usize,Image)>){
        let img_iter=imgs.iter();
        todo!();

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

    pub fn continuous_text(&self) -> Option<String> {
        let res = self
            .0
            .iter()
            .map(|t| {
                if t.get_type().to_lowercase() == "plain" {
                    Some(t.into_target::<Plain>().unwrap().text.clone())
                } else {
                    None
                }
            })
            .reduce(|a, b| {
                if a.is_some() && b.is_some() {
                    Some(format!("{}{}", a.unwrap(), b.unwrap()))
                } else {
                    None
                }
            })
            .and_then(|f| if f.is_some() { Some(f.unwrap()) } else { None });

        res
    }
}
