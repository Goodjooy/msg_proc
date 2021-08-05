use std::fmt::Display;

use msg_chain::MessageChain;

use crate::rev::msg_chain::{At, AtAll, Face, FlashImage, Image, Plain, Voice};

#[macro_export]
macro_rules! chain_generate {
    (xml : $s:expr) => {
        Box::new(chain::msg_chain::Xml {
            xml: $s.to_string(),
        })
    };
    (json : $s:expr) => {
        Box::new(chain::msg_chain::Json {
            json: $s.to_string(),
        })
    };
    (app : $s:expr) => {
        Box::new(chain::msg_chain::App {
            content: $s.to_string(),
        })
    };
}

pub enum ResouceSrc<'a> {
    Id(&'a str),
    Path(&'a str),
    Url(&'a str),
    Base64(&'a str),
}

impl<'a> ResouceSrc<'a> {
    pub fn id(id: &'a str) -> Self {
        Self::Id(id)
    }
    pub fn path(path: &'a str) -> Self {
        Self::Path(path)
    }
    pub fn url(url: &'a str) -> Self {
        Self::Url(url)
    }
    pub fn base64(base: &'a str) -> Self {
        Self::Base64(base)
    }

    pub fn get_value(self) -> String {
        match self {
            ResouceSrc::Id(s)
            | ResouceSrc::Path(s)
            | ResouceSrc::Url(s)
            | ResouceSrc::Base64(s) => s.to_string(),
        }
    }
}

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

const send_unable_type: [&str; 2] = ["Quate", "Source"];

impl ChainBuilder {
    pub fn into_sendable(self) -> Self {
        let v = self
            .0
            .into_iter()
            .filter(|f| !send_unable_type.contains(&f.get_type()))
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

type BulidTarget = Vec<Box<dyn MessageChain>>;
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

    pub fn xml(mut self, xml: String) -> BulidTarget {
        todo!()
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
