use std::fmt::Display;

use msg_chain::MessageChain;

use crate::rev::msg_chain::{At, AtAll, Face, FlashImage, Image, Plain, Voice};

use super::{BulidTarget, resouce::ResouceSrc};


pub struct ChainBuilder(Vec<Box<dyn MessageChain>>);

impl ChainBuilder {
    pub fn new() -> Self {
        Self(vec![])
    }
    pub fn build(self) -> Vec<Box<dyn MessageChain>> {
        self.0
    }

    pub fn form_chain<I>(source: I) -> Self
    where
        I: Iterator<Item = Box<dyn MessageChain>>,
    {
        let vec = source.collect::<Vec<_>>();
        Self(vec)
    }
}

pub const SEND_UNABLE_TYPE: [&str; 2] = ["Quate", "Source"];

impl ChainBuilder {
    pub fn into_sendable(self) -> Self {
        let v = self
            .0
            .into_iter()
            .filter(|f| !SEND_UNABLE_TYPE.contains(&f.get_type()))
            .collect::<Vec<_>>();
        Self(v)
    }

    pub fn check_conflict(&self) -> bool {
        todo!()
    }
}

impl ChainBuilder {
    pub fn textln<T: Display>(mut self, text: T) -> Self {
        self.0.push(Box::new(Plain {
            text: format!("{}\n", text),
        }));
        self
    }

    pub fn text<T: Display>(mut self, text: T) -> Self {
        self.0.push(Box::new(Plain {
            text: text.to_string(),
        }));
        self
    }

    pub fn image(mut self, source: ResouceSrc) -> Self {
        let mut img = Image::default();
        match source {
            ResouceSrc::Id(_) => img.image_id = Some(source.get_value()),
            ResouceSrc::Path(_) => img.path = Some(source.get_value()),
            ResouceSrc::Url(_) => img.url = Some(source.get_value()),
            ResouceSrc::Base64(_) => img.base64 = Some(source.get_value()),
        };

        self.0.push(Box::new(img));
        self
    }

    pub fn face(mut self, id: i32) -> Self {
        self.0.push(Box::new(Face {
            face_id: id,
            name: String::new(),
        }));
        self
    }

    pub fn at(mut self, target: u64) -> Self {
        let res = At {
            target,
            display: String::new(),
        };
        self.0.push(Box::new(res));
        self
    }

    pub fn at_all(mut self) -> Self {
        self.0.push(Box::new(AtAll));
        self
    }
}


impl ChainBuilder {
    pub fn flash_image(mut self, source: ResouceSrc) -> BulidTarget {
        let mut img = FlashImage::default();
        match source {
            ResouceSrc::Id(_) => img.image_id = Some(source.get_value()),
            ResouceSrc::Path(_) => img.path = Some(source.get_value()),
            ResouceSrc::Url(_) => img.url = Some(source.get_value()),
            ResouceSrc::Base64(_) => img.base64 = Some(source.get_value()),
        };

        self.0.push(Box::new(img));
        self.build()
    }

    pub fn voice(mut self, source: ResouceSrc) -> BulidTarget {
        let mut img = Voice::default();
        match source {
            ResouceSrc::Id(_) => img.voice_id = Some(source.get_value()),
            ResouceSrc::Path(_) => img.path = Some(source.get_value()),
            ResouceSrc::Url(_) => img.url = Some(source.get_value()),
            ResouceSrc::Base64(_) => img.base64 = Some(source.get_value()),
        };

        self.0.push(Box::new(img));
        self.build()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_chain_builder() {
        let chain = ChainBuilder::new()
            .textln("开始")
            .image(ResouceSrc::path("./static/test_img.png"))
            .text("好耶")
            .face(53)
            .at(114145)
            .build();

        let chain = ChainBuilder::form_chain(chain.into_iter())
            .textln("好耶2")
            .build();

        let res = serde_json::to_string_pretty(&chain);

        println!("{}", &res.unwrap())
    }
}
