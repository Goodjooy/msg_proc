# Msg Procces

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

## 附加

* 判断当前消息类型
    * 消息
    * 事件
    * 发送消息响应

* 消息sender
    * 多种共用
    * sender--target 转换
* 消息target
    * 多种target
    * 接收msg_chain
    * 序列化后放入容器即可使用