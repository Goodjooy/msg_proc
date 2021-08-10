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
    pub fn if_else<F, FE>(self, status: bool, f: F, f_else: FE) -> Self
    where
        F: FnOnce(Self) -> Self,
        FE: FnOnce(Self) -> Self,
    {
        if status {
            f(self)
        } else {
            f_else(self)
        }
    }
}
/// option branch statments
impl ChainBuilder {
    pub fn if_some<T, FS>(self, data: Option<T>, handle_some: FS) -> Self
    where
        FS: FnOnce(Self, T) -> Self,
    {
        match data {
            Some(data) => handle_some(self, data),
            None => self,
        }
    }
    pub fn if_none<T, FN>(self, data: Option<T>, handle_none: FN) -> Self
    where
        FN: FnOnce(Self) -> Self,
    {
        match data {
            Some(_) => self,
            None => handle_none(self),
        }
    }
    pub fn if_option<T, FS, FN>(self, data: Option<T>, handle_some: FS, handle_none: FN) -> Self
    where
        FS: FnOnce(Self, T) -> Self,
        FN: FnOnce(Self) -> Self,
    {
        match data {
            Some(data) => handle_some(self, data),
            None => handle_none(self),
        }
    }
}

impl ChainBuilder {
    pub fn if_ok<T, FO, E>(self, data: Result<T, E>, handle: FO) -> Self
    where
        FO: FnOnce(Self, T) -> Self,
    {
        match data {
            Ok(data) => handle(self, data),
            Err(_err) => self,
        }
    }
    pub fn if_err<T, E, FE>(self, data: Result<T, E>, err_handle: FE) -> Self
    where
        FE: FnOnce(Self, E) -> Self,
    {
        match data {
            Ok(_) => self,
            Err(err) => err_handle(self, err),
        }
    }
    pub fn if_result<T, FO, E, FE>(self, data: Result<T, E>, handle: FO, err_handle: FE) -> Self
    where
        FO: FnOnce(Self, T) -> Self,
        FE: FnOnce(Self, E) -> Self,
    {
        match data {
            Ok(data) => handle(self, data),
            Err(err) => err_handle(self, err),
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
        if let Some(t) = text {
            data.push(Box::new(Plain { text: t }));
            text = None;
        }

        self.0 = data;
        self
    }

    pub fn check_conflict(&self) -> bool {
        todo!()
    }
}

impl ChainBuilder {
    pub fn push(mut self, data: Box<dyn MessageChain>) -> Self {
        self.0.push(data);
        self
    }

    pub fn do_operate<F>(self, handle: F) -> Self
    where
        F: FnOnce(Self) -> Self,
    {
        handle(self)
    }

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

    pub fn text_repeat<T: Display>(self, text: T, times: usize) -> Self {
        let s = text.to_string();
        let s = s.repeat(times);

        self.text(s)
    }

    pub fn text_repeat_ln<T: Display>(self, text: T, times: usize) -> Self {
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
            .text("好耶")
            .textln("开始")
            .text_repeat("emm", 4)
            .text_repeat_ln("-", 6)
            .image(ResouceSrc::path("./static/test_img.png"))
            .face(53)
            .at(114145)
            .at_all()
            .push(Box::new(Plain {
                text: String::from("abab"),
            }))
            .if_then(true, |chain| {
                chain
                    .textln("yes")
                    .at_all()
                    .face(13)
                    .textln("不对吧")
                    .image(ResouceSrc::url("http://idididid"))
            })
            .loop_in_with_sep(
                0..4,
                |chain, data: u8, i| {
                    chain
                        .if_then(data % 2 == 0, |chain| chain.textln("现在是偶数"))
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

    #[test]
    fn if_branch() {
        let so = Some(12);
        let so2 = Some(11);
        let ne = Option::<u8>::None;
        let ok = Result::<i32, i32>::Ok(12);
        let err = Result::<i32, i32>::Err(12);
        let err2 = Result::<i32, i32>::Err(113);

        let chain = ChainBuilder::new()
            // 当条件为true,使用闭包，否则不进行任何操作
            .if_then(2 + 2 == 4, |chain| chain.textln("2+2=4 is true"))
            // 当条件为true,使用闭包f，否则使用闭包f_else
            .if_else(
                1 * 12 == 4,
                |chain| chain.text("1*12 是 14"),
                |chain| chain.text("1*12 不是 14"),
            )
            // 当 传入 Option 为 Some(T)时，使用闭包 handle_some ，否则不做任何操作
            .if_some(so, |chain, data| chain.text(format!("ok: ->{}", data)))
            // 当 传入 Option 为 None时，使用闭包 handle_none ，否则不做任何操作
            .if_none(ne, |chain| chain.text("err"))
            // 当 传入 Option 为 Some(T)时，使用闭包 handle_some ，否则使用闭包 handle_none
            .if_option(
                so2,
                |chain, data| chain.text(data),
                |chain| chain.text("is None"),
            )
            // 当 传入 Result 为 Ok(T)时，使用闭包 hanle ，否则不做任何操作
            .if_ok(ok, |chain, data| chain.text(data))
            // 当 传入 Result 为 Err(E)时，使用闭包 err_hanle ，否则不做任何操作
            .if_err(err, |chain, err| chain.text("err:").text(err))
            // 当 传入 Result 为 Ok(T) 时，使用闭包 hanle ，否则使用闭包 err_hanle
            .if_result(
                err2,
                |chain, data| chain.text("ok ").text(data),
                |chain, err| chain.text("err ").text(err),
            );
    }

    #[test]
    fn loop_statement() {
        let iter1 = 1..5;
        let iter2 = 'a'..'c';

        let chain = ChainBuilder::new()
            //在迭代遍历每个元素时，在2个元素间使用sep构造分割线
            .loop_in_with_sep(
                iter1,
                |chain, data, index| chain.text(format!("第{}个。是： {}", index, data)),
                |chain| chain.text_repeat_ln("-", 6),
            )
            // 对每个元素调用handle
            .loop_in(iter2, |chain, data, _| {
                chain.text(format!("当前是字符：{}", data))
            });
    }


    #[test]
    fn test_complex() {
        let a=Some(1);

        let chain=ChainBuilder::new()
        .do_operate(|chain|match a {
            Some(d) => chain.face(d),
            None => chain.face(112),
        });
    }
}
