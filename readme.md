# Msg Process

* 负责[mirai-api-http](https://github.com/project-mirai/mirai-api-http)中的接收消息反序列化和发送消息的序列化
* 定义消息结构

## 接收消息

* [群消息](https://github.com/project-mirai/mirai-api-http/blob/master/docs/api/MessageType.md#%E7%BE%A4%E6%B6%88%E6%81%AF)
* [好友消息](https://github.com/project-mirai/mirai-api-http/blob/master/docs/api/MessageType.md#%E5%A5%BD%E5%8F%8B%E6%B6%88%E6%81%AF)
* [群临时会话消息](https://github.com/project-mirai/mirai-api-http/blob/master/docs/api/MessageType.md#%E7%BE%A4%E4%B8%B4%E6%97%B6%E6%B6%88%E6%81%AF)
* [陌生人消息](https://github.com/project-mirai/mirai-api-http/blob/master/docs/api/MessageType.md#%E9%99%8C%E7%94%9F%E4%BA%BA%E6%B6%88%E6%81%AF)
* [客户端间通讯](https://github.com/project-mirai/mirai-api-http/blob/master/docs/api/MessageType.md#%E5%85%B6%E4%BB%96%E5%AE%A2%E6%88%B7%E7%AB%AF%E6%B6%88%E6%81%AF)

## 发送消息

* [群消息](https://github.com/project-mirai/mirai-api-http/blob/master/docs/adapter/WebsocketAdapter.md#%E5%8F%91%E9%80%81%E7%BE%A4%E6%B6%88%E6%81%AF)
* [好友消息](https://github.com/project-mirai/mirai-api-http/blob/master/docs/adapter/WebsocketAdapter.md#%E5%8F%91%E9%80%81%E5%A5%BD%E5%8F%8B%E6%B6%88%E6%81%AF)
* [临时会话消息](https://github.com/project-mirai/mirai-api-http/blob/master/docs/adapter/WebsocketAdapter.md#%E5%8F%91%E9%80%81%E4%B8%B4%E6%97%B6%E4%BC%9A%E8%AF%9D%E6%B6%88%E6%81%AF)
* [戳一戳消息](https://github.com/project-mirai/mirai-api-http/blob/master/docs/adapter/WebsocketAdapter.md#%E5%8F%91%E9%80%81%E5%A4%B4%E5%83%8F%E6%88%B3%E4%B8%80%E6%88%B3%E6%B6%88%E6%81%AF)
* [消息撤回](https://github.com/project-mirai/mirai-api-http/blob/master/docs/adapter/WebsocketAdapter.md#%E6%92%A4%E5%9B%9E%E6%B6%88%E6%81%AF)

## 接收消息反序列化

* `MessageRev`

*

```rust
    pub struct MessageRev {
    pub msg_type: String,  // 消息类型
    pub sender: Box<dyn Sender>, // 消息发送者
    pub chain: Vec<Box<dyn MessageChain>>, // 消息内容
    }
    ```

* 提供方法 `load_recive_data`将解析完毕的`Hashmap<String,serde_json::Value>`转换为`MessageRev`

## 发送消息序列化

* todo

## 消息链构造器

* 消息链类型为`Vec<Box<dyn MessageChain>>`,手动构造消息链是比较繁琐的。提供`ChainBuiler`用于辅助构造消息链
* 创建构造器

```rust
    let chain = ChainBuilder::new();
```

* 为常用消息链提供构造支持

```rust
    let chain : ChainBuilder = ChainBuilder::new()
        .text("好耶")           // 文本
        .textln("开始")         // 自动换行文本
        .text_repeat("emm", 4)  // 将指定文本重复 rep次
        .text_repeat_ln("-", 6) // 将指定文本重复 rep次 并自动换行

        .image(ResouceSrc::path("./static/test_img.png")) // 图片
        .face(53)   //表情
        .at(114145) // At 指定成员
        .at_all()   // At 全体成员

        // 对于没有实现助构造器的，可以手动添加
        .push(Box::new(Plain{text:String::from("abab")}));   
```

* 分支语句构造支持

```rust
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
    // 当 传入 Option 为 Some(T)时，使用闭包 handle_some ，否则使用闭包handle_none
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
```

* 循环语持

```rust
    iter1 = 1..5;
    let iter2 = 'a'..'c';

    let chain = ChainBuilder::new()
    // 对每个元素调用handle
    .loop_in(iter2, |chain, data, _| {
        chain.text(format!("当前是字符：{}", data))
    })
    //在迭代遍历每个元素时，在2个元素间使用sep构造分割线
    .loop_in_with_sep(
        iter1,
        |chain, data, index| chain.text(format!("第{}个。是： {}", index, data)),
        |chain| chain.text_repeat_ln("-", 6),
    );
```

* 更加复杂的逻辑语句和操作

```rust
    let a=Some(1);

    let chain=ChainBuilder::new()
    .do_operate(|chain|match a {
        Some(d) => chain.face(d),
        None => chain.face(112),
    });
```
