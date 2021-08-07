use std::fmt::Display;

use msg_chain::MessageChain;

use crate::rev::msg_chain::{At, AtAll, Face, FlashImage, Image, Plain, Voice};

use super::{resouce::ResouceSrc, BulidTarget};

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
impl ChainBuilder {
    pub fn if_then<F>(self, status: bool, f: F) -> Self
    where
        F: FnOnce(Self) -> Self,
    {
        if status {
            f(self)
        } else {
            self
        }
    }

    pub fn if_some<T, F>(self, data: Option<T>, handle: F) -> Self
    where
        F: FnOnce(Self, T) -> Self,
    {
        match data {
            Some(t) => handle(self, t),
            None => self,
        }
    }
    pub fn default_err_handle<E>(self, _err: E) -> Self {
        self
    }
    pub fn if_ok<T, F, E, FE>(self, data: Result<T, E>, handle: F, err_handle: FE) -> Self
    where
        F: FnOnce(Self, T) -> Self,
        FE: FnOnce(Self, E) -> Self,
    {
        match data {
            Ok(data) => handle(self, data),
            Err(err) => err_handle(self, err),
        }
    }

    pub fn loop_in<T, F, I, F_S>(mut self, src: I, handle: F, sep: F_S) -> Self
    where
        I: Iterator<Item = T>,
        F: Fn(Self, T, usize) -> Self,
        F_S: Fn(Self) -> Self,
    {
        let mut start = false;
        for (index, data) in src.enumerate() {
            self = handle(self, data, index);
            if start {
                self = sep(self)
            }
            start = true;
        }
        self
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

    pub fn text_repeat<T: Display>(mut self, text: T, times: usize) -> Self {
        let s = text.to_string();
        let s = s.repeat(times);

        self.text(s)
    }

    pub fn text_repeat_ln<T: Display>(mut self, text: T, times: usize) -> Self {
        let s = text.to_string();
        let s = s.repeat(times);

        self.textln(s)
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
            .if_then(true, |b| {
                b.textln("yes")
                    .at_all()
                    .face(13)
                    .text("不对吧")
                    .image(ResouceSrc::url("http://idididid"))
            })
            .loop_in(
                vec![1, 2, 3, 4].iter(),
                |f, b, _i| f.textln(format!("好耶：：{}", b)).at(114145),
                |s| s.text_repeat_ln("-", 6),
            )
            .build();

        let chain = ChainBuilder::form_chain(chain.into_iter())
            .textln("好耶2")
            .build();

        let res = serde_json::to_string_pretty(&chain);

        println!("{}", &res.unwrap())
    }
}
