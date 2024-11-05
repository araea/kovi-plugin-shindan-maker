# kovi-plugin-shindan-maker

[<img alt="github" src="https://img.shields.io/badge/github-araea/kovi_plugin_shindan_maker-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/araea/kovi-plugin-shindan-maker)
[<img alt="crates.io" src="https://img.shields.io/crates/v/kovi-plugin-shindan-maker.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/kovi-plugin-shindan-maker)

Kovi 的 [ShindanMaker](https://en.shindanmaker.com/) 占卜插件。内置 690+ 占卜项。

## 前置

1. 创建 Kovi 项目。
2. 执行 `cargo kovi add shindan-maker`。
3. 在 `src/main.rs` 中添加 `kovi_plugin_shindan_maker`。

## 使用

1. 发送 `神断列表` 查看神断命令列表。
2. 发送神断命令，如 `抽老婆`，即可生成神断结果。
3. 参数 `-t` 或 `-i` 可指定模式（文/图）。
4. 结合 `神断帮助` 与 `-h` 参数自行探索。

## 配置

* 资源目录 - `data/kovi-plugin-shindan-maker/*`
    * 第一次运行时自动生成。

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

# 指令前缀
# 示例：["!", "。"]
prefixes = []

# 回复时 @
is_at = true

# 回复时引用
is_quote = false

# 神断网区域
# 可选："Jp", "En", "Cn", "Kr", "Th"
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

* 添加 - 使用 `添加神断` 指令。
* 删除 - 直接编辑或使用 `删除神断` 指令。

## 致谢

* [Kovi](https://kovi.threkork.com/)
* [ShindanMaker](https://cn.shindanmaker.com/)

<br>

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>

