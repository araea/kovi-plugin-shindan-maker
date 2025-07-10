# kovi-plugin-shindan-maker

[![github](https://img.shields.io/badge/github-araea/kovi_plugin_shindan_maker-8da0cb?style=for-the-badge&labelColor=555555&logo=github)](https://github.com/araea/kovi-plugin-shindan-maker)
[![crates.io](https://img.shields.io/crates/v/kovi-plugin-shindan-maker.svg?style=for-the-badge&color=fc8d62&logo=rust)](https://crates.io/crates/kovi-plugin-shindan-maker)

Kovi 的 [ShindanMaker](https://en.shindanmaker.com/) 占卜插件。
内置 690+ 占卜项。

## 前置

1. 创建 Kovi 项目
2. 执行 `cargo kovi add shindan-maker`
3. 在 `src/main.rs` 中添加 `kovi_plugin_shindan_maker`

## 使用

1. 发送 `神断列表` 查看所有指令
2. 发送指令，如 `抽老婆`，生成结果
3. 使用 `-t` 或 `-i` 参数指定模式（文 / 图）
4. 结合 `神断帮助` 与 `-h` 自行探索

> 注意：若无法请求神断网，请使用 VPN。

## 配置

资源目录 : `data/kovi-plugin-shindan-maker/*`
> 首次运行时自动生成。

### `config.toml` - 插件配置

```toml
# 频道过滤
[channel]
# 频道白名单（只允许）
white = ["123456789", "987654321"]
# 频道黑名单（排除）
black = ["123456789"]

# 插件配置
[plugin]
# 仅 @ 响应
only_at = false
# 指令前缀, 示例：["!", "。"]
prefixes = []
# 回复时 @
is_at = true
# 回复时引用
is_quote = false
# 神断网区域, 可选: "Jp", "En", "Cn", "Kr", "Th"
domain = "Cn"
# `随机神断` 时提示命令
random_return_command = true
# 排行榜最多显示
rank_max = 30
```

### `commands.toml` - 指令配置

```toml
[[command]]
# 功能（勿改）
function = "插件指令列表"
# 指令名（可增删）
commands = ["神断帮助", "bz"]
```

### `shindans.toml` - 占卜项

添加 : 使用 `添加神断` 指令
删除 : 编辑文件或使用 `删除神断` 指令

## 致谢

- [Kovi](https://kovi.threkork.com/)
- [ShindanMaker](https://cn.shindanmaker.com/)

---

### License

_Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option._

_Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions._
