use std::fmt::Display;

use msg_chain::MessageChain;

use crate::rev::msg_chain::{At, AtAll, Face, FlashImage, Image, Plain, Voice};

use super::{resouce::ResouceSrc, BulidTarget};

pub struct ChainBuilder(Vec<Box<dyn MessageChain>>);

/// constructors
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
/// if branch statment
impl ChainBuilder {
    pub fn if_then<F>(self, status: bool, f: F, f_else: Option<F>) -> Self
    where
        F: FnOnce(Self) -> Self,
    {
        if status {
            f(self)
        } else {
            if let Some(f_else) = f_else {
                f_else(self)
            } else {
                self
            }
        }
    }
}
/// option branch statments
impl ChainBuilder {
    pub fn if_option<T, FS, FN>(
        self,
        data: Option<T>,
        handle_some: Option<FS>,
        handle_none: Option<FN>,
    ) -> Self
    where
        FS: FnOnce(Self, T) -> Self,
        FN: FnOnce(Self) -> Self,
    {
        match data {
            Some(data) => {
                if let Some(handle_some) = handle_some {
                    handle_some(self, data)
                } else {
                    self
                }
            }
            None => {
                if let Some(handle_none) = handle_none {
                    handle_none(self)
                } else {
                    self
                }
            }
        }
    }
}

impl ChainBuilder {
    pub fn if_result<T, FO, E, FE>(
        self,
        data: Result<T, E>,
        handle: Option<FO>,
        err_handle: Option<FE>,
    ) -> Self
    where
        FO: FnOnce(Self, T) -> Self,
        FE: FnOnce(Self, E) -> Self,
    {
        match data {
            Ok(data) => {
                if let Some(handle) = handle {
                    handle(self, data)
                } else {
                    self
                }
            }
            Err(err) => {
                if let Some(handle) = err_handle {
                    handle(self, err)
                } else {
                    self
                }
            }
        }
    }
}

impl ChainBuilder {
    pub fn loop_in_with_sep<T, F, I, FS>(mut self, src: I, handle: F, sep: FS) -> Self
    where
        I: Iterator<Item = T>,
        F: Fn(Self, T, usize) -> Self,
        FS: Fn(Self) -> Self,
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
    pub fn loop_in<T, F, I>(mut self, src: I, handle: F) -> Self
    where
        I: Iterator<Item = T>,
        F: Fn(Self, T, usize) -> Self,
    {
        for (index, data) in src.enumerate() {
            self = handle(self, data, index);
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

    pub fn simplify(mut self) -> Self {
        let mut data: Vec<Box<dyn MessageChain>> = Vec::new();
        let mut text = None;

        for d in self.0 {
            if d.get_type().to_lowercase() == "plain" {
                let t = d.into_target::<Plain>().unwrap();
                text = match text {
                    Some(s) => Some(format!("{}{}", s, t.text)),
                    None => Some(t.text),
                }
            } else {
                if let Some(t) = text {
                    data.push(Box::new(Plain { text: t }));
                    text = None;
                }
                data.push(d)
            }
        }
        self.0 = data;
        self
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
    use crate::send::utils::chain_handle::ToMsgHandle;

    use super::*;

    #[test]
    fn test_chain_builder() {
        let chain = ChainBuilder::new()
            .textln("开始")
            .image(ResouceSrc::path("./static/test_img.png"))
            .text("好耶")
            .face(53)
            .at(114145)
            .if_then(
                true,
                |chain| {
                    chain
                        .textln("yes")
                        .at_all()
                        .face(13)
                        .textln("不对吧")
                        .image(ResouceSrc::url("http://idididid"))
                },
                None,
            )
            .loop_in_with_sep(
                0..4,
                |chain, data: u8, i| {
                    chain
                        .if_then(data % 2 == 0, |chain| chain.textln("现在是偶数"), None)
                        .textln(format!(" 好耶：：{} |当前第{}个 ", data, i))
                        .text_repeat_ln("ab", 5)
                        .at(114145)
                },
                |chain| chain.text_repeat_ln("-", 6),
            )
            .simplify()
            .build();

        let res = serde_json::to_string_pretty(&chain);

        println!("{}", &res.unwrap());
        println!("{}", chain.to_msg_handle().conbin_plain().unwrap())
    }
}
