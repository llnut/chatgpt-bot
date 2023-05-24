# chatgpt-bot

为你的微信/QQ对接chatgpt服务。



## 目录

- [安装](#安装)
- [配置](#配置)
- [使用](#使用)
- [特性](#特性)
- [License](#license)

## 安装
安装最新版本[Stable Rust](https://www.rust-lang.org/tools/install)
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
安装最新版本[Cross](https://github.com/cross-rs/cross)，并拉取编译镜像(可选，跨平台编译用)
```bash
cargo install cross --git https://github.com/cross-rs/cross
docker pull ghcr.io/cross-rs/x86_64-pc-windows-gnu:edge
```

在linux/windows中编译chatgpt-bot
```bash
cd /path/to/chatgpt-bot
cargo build --release
```

linux为windows编译chatgpt-bot
```bash
cross build --release --target x86_64-pc-windows-gnu
```

## 配置
配置文件需放置在chatgpt-bot可执行文件的同级目录中，可将本仓库中的Config.toml.example复制一份，
以下是示例配置，其中的`api_key`需修改成你的chatgpt api key，并按照实际情况修改鲲鹏机器人的ip以及端口。

```toml
ip = "0.0.0.0"
http_port = 3000
server_name = "chatgpt-bot"
server_type = "KPBackend" # The possible values are: KPBackend / CQBackend

[rate_limit]
capacity = 1
quantum = 1
rate = 1

[openai]
open = true
stream = true
api_key = "" # Your openai api key
api_domain = "https://api.openai.com"
max_tokens = 300

[handler.kp]
ip = "127.0.0.1" # KP wechat robot ip
port = 2022

[handler.cq]
ip = "127.0.0.1"
port = 5700

[reply]
prefix = "#"
cache_per_question = 1
blacklist = ["钱包", "支付宝", "微信", "收款", "收钱", "给我转账", "转给我", "分销", "付款", "售价", "chatgpt", "openai"]

[reply.replace]
"AI语言模型" = ["人工智障"]
"AI language model" = ["AI idiot"]
"Artificial Intelligence" = ["Artificial idiot"]
"AI Intelligence" = ["AI idiot"]
"AI language mod" = ["AI idiot"]
"人工智能" = ["人工智障"]
"语言模型" = [""]

[reply.text]
hello = ["你好呀", "hello"]
"卧槽" = ["不要说脏话哟", "你再这样我生气啦"]
"llnut" = ["那是主人的名字", "主人~"]
"llnut是谁" = ["我的主人哟", "是主人哦"]

[reply.static_picture]
# empty

[reply.gif_picture]
# empty

[log]
level = "debug"
with_thread_ids = false
with_thread_names = false
```

## 使用
### 微信-[鲲鹏机器人](https://www.kunpeng.cf/)
适用于鲲鹏微信机器人的chatgpt服务，需使用kp-http插件完成对接

1. 启动chatgpt-bot
1. 安装kp-http框架并启动鲲鹏机器人
2. 在kp-http框架中设置消息推送地址为http://{chatgpt-bot的ip}:3001
3. 对微信机器人发送消息，即可得到chatgpt的回复(默认设置中，发送消息需加上"#"前缀以防止在微信群中机器人胡乱回复。例如：#你好)

### QQ-[go-cqhttp](https://github.com/Mrs4s/go-cqhttp)
尚未支持

## 特性
- [x] 消息缓存
- [x] 限流控制(仅支持linux)
- [x] 基于关键词的回复消息替换
- [x] 基于关键词的接收消息黑名单
- [x] 默认回复
- [x] 微信支持
- [x] gpt3.5 api
- [ ] gpt4 api
- [ ] 上下文记忆
- [ ] QQ支持


## License
[MIT](LICENSE) © llnut
