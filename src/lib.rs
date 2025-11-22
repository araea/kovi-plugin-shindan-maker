//! This is a single-file version of the kovi-plugin-shindan-maker crate.
//! All modules have been merged into this lib.rs file.

// --- START OF MODULE: types ---
mod types {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub(crate) struct Config {
        pub(crate) channel: Channel,
        pub(crate) plugin: Plugin,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub(crate) struct Channel {
        pub(crate) white: Vec<String>,
        pub(crate) black: Vec<String>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub(crate) struct Plugin {
        pub(crate) only_at: bool,
        pub(crate) prefixes: Vec<String>,
        pub(crate) is_at: bool,
        pub(crate) is_quote: bool,
        pub(crate) domain: String,
        pub(crate) random_return_command: bool,
        pub(crate) rank_max: u32,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub(crate) struct Commands {
        pub(crate) command: Vec<Command>,
    }

    impl Commands {
        pub(crate) fn get_command(&self, command: &str) -> Option<&Command> {
            self.command
                .iter()
                .find(|c| c.commands.contains(&String::from(command)))
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub(crate) struct Command {
        pub(crate) function: String,
        pub(crate) commands: Vec<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub(crate) struct Shindans {
        pub(crate) shindan: Vec<Shindan>,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub(crate) struct Shindan {
        pub(crate) id: String,
        pub(crate) title: String,
        pub(crate) description: String,
        pub(crate) command: String,
        pub(crate) mode: String,
        pub(crate) count: u32,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub(crate) struct UserDatas {
        pub(crate) user: Vec<UserData>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub(crate) struct UserData {
        pub(crate) id: String,
        pub(crate) name: String,
        pub(crate) count: u32,
    }
}
// --- END OF MODULE: types ---

// --- START OF MODULE: init_data ---
mod init_data {
    pub(crate) const CONFIG_TOML: &str = r#"
# 频道过滤
[channel]
# 频道白名单（只允许）
white = []

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
domain = "Jp"

# `随机神断` 时提示命令
random_return_command = true

# 排行榜最多显示
rank_max = 20
"#;

    pub(crate) const COMMANDS_TOML: &str = r#"
[[command]]
function = "插件指令列表"
commands = ["神断帮助"]

[[command]]
function = "添加神断命令"
commands = ["添加神断"]

[[command]]
function = "删除神断命令"
commands = ["删除神断"]

[[command]]
function = "随机神断命令"
commands = ["随机神断"]

[[command]]
function = "神断命令列表"
commands = ["神断列表"]

[[command]]
function = "设置神断模式"
commands = ["设置神断"]

[[command]]
function = "修改神断命令"
commands = ["修改神断"]

[[command]]
function = "查看用户神断次数"
commands = ["用户次数"]

[[command]]
function = "用户神断次数排行榜"
commands = ["用户排行榜"]

[[command]]
function = "查看神断信息"
commands = ["查看神断"]

[[command]]
function = "神断被触发次数排行榜"
commands = ["神断次数"]

[[command]]
function = "模糊查找神断命令"
commands = ["查找神断"]
"#;

    pub(crate) const USER_DATA_TOML: &str = r#"
[[user]]
id = "123456"
name = "神秘人"
count = 0
"#;

    pub(crate) const SHINDANS_TOML: &str = include_str!("../res/shindans.toml");
}
// --- END OF MODULE: init_data ---

// --- START OF MODULE: data ---
mod data {
    use kovi::utils;
    use kovi::{PluginBuilder, toml};
    use std::path::PathBuf;
    use std::sync::RwLock;

    use super::init_data::{COMMANDS_TOML, CONFIG_TOML, SHINDANS_TOML, USER_DATA_TOML};
    use super::types::{Commands, Config, Shindan, Shindans, UserDatas};

    #[derive(Debug)]
    pub(crate) struct Data {
        data_path: PathBuf,
        pub(crate) config: Config,
        pub(crate) commands: Commands,
        pub(crate) shindans: RwLock<Shindans>,
        pub(crate) user_data: RwLock<UserDatas>,
    }

    impl Data {
        pub(crate) fn new() -> Self {
            let bot = PluginBuilder::get_runtime_bot();
            let data_path = bot.get_data_path();

            let config_path = data_path.join("config.toml");
            let default_config: Config = toml::from_str(CONFIG_TOML).unwrap();
            let config = utils::load_toml_data(default_config, config_path).unwrap();

            let commands_path = data_path.join("commands.toml");
            let default_commands: Commands = toml::from_str(COMMANDS_TOML).unwrap();
            let commands = utils::load_toml_data(default_commands, commands_path).unwrap();

            let shindans_path = data_path.join("shindans.toml");
            let default_shindans: Shindans = toml::from_str(SHINDANS_TOML).unwrap();
            let shindans = utils::load_toml_data(default_shindans, shindans_path).unwrap();

            let user_data_path = data_path.join("user_data.toml");
            let default_user_data: UserDatas = toml::from_str(USER_DATA_TOML).unwrap();
            let user_data = utils::load_toml_data(default_user_data, user_data_path).unwrap();

            Self {
                data_path,
                config,
                commands,
                shindans: RwLock::new(shindans),
                user_data: RwLock::new(user_data),
            }
        }

        pub(crate) fn save(&self) {
            let data_path = &self.data_path;

            let shindans_path = data_path.join("shindans.toml");

            let mut shindans_data = self.shindans.write().unwrap();
            shindans_data
                .shindan
                .sort_by(|a, b| a.command.cmp(&b.command));
            let shindans_data: &Shindans = &shindans_data;
            utils::save_toml_data(shindans_data, shindans_path).unwrap();

            let user_data_path = data_path.join("user_data.toml");
            let user_data_data: &UserDatas = &self.user_data.read().unwrap();
            utils::save_toml_data(user_data_data, user_data_path).unwrap();
        }

        pub(crate) fn add_shindan(
            &self,
            id: &str,
            title: &str,
            description: &str,
            command: &str,
            mode: &str,
        ) {
            let shindan = Shindan {
                id: id.to_string(),
                title: title.to_string(),
                description: description.to_string(),
                command: command.to_string(),
                mode: mode.to_string(),
                count: 0,
            };

            let mut shindans = self.shindans.write().unwrap();
            shindans.shindan.push(shindan);
        }
    }
}
// --- END OF MODULE: data ---

// --- START OF MODULE: plugin_utils ---
mod plugin_utils {
    use anyhow::Result;
    use cdp_html_shot::Browser;
    use kovi::bot::message::Segment;
    use kovi::serde_json::json;
    use kovi::tokio::time;
    use kovi::{Message, MsgEvent, RuntimeBot, log};
    use rand::prelude::SliceRandom;
    use shindan_maker::{Segments, ShindanClient};
    use std::sync::Arc;

    use super::data::Data;
    use super::types::{Config, UserData};

    const SHINDAN_ERROR_MSG: &str = "神断失败: 神累了";

    pub(crate) fn should_process_group(
        group_id: Option<&i64>,
        white_list: &[String],
        black_list: &[String],
    ) -> bool {
        let group_id = match group_id {
            Some(id) => id.to_string(),
            None => return true,
        };

        if black_list.contains(&group_id) {
            return false;
        }

        if !white_list.is_empty() && !white_list.contains(&group_id) {
            return false;
        }

        true
    }

    pub(crate) fn parse_command<'a>(
        text: &'a str,
        prefixes: &[String],
    ) -> Option<(&'a str, Vec<&'a str>)> {
        let mut words: Vec<&str> = text.split_whitespace().collect();
        if words.is_empty() {
            return None;
        }

        let mut command = words.remove(0);

        if prefixes.is_empty() {
            Some((command, words))
        } else {
            let mut sorted_prefixes = prefixes.to_vec();
            sorted_prefixes.sort_by_key(|b| std::cmp::Reverse(b.len()));

            command = sorted_prefixes
                .iter()
                .find(|&p| command.starts_with(p))
                .map(|p| &command[p.len()..])?;

            Some((command, words))
        }
    }

    pub(crate) fn build_and_send_message(event: &Arc<MsgEvent>, data: &Arc<Data>, msg: &str) {
        let message = match (data.config.plugin.is_at, data.config.plugin.is_quote) {
            (true, false) => Message::new()
                .add_at(&event.user_id.to_string())
                .add_text("\n")
                .add_text(msg),
            (false, true) => Message::new().add_reply(event.message_id).add_text(msg),
            (true, true) => Message::new()
                .add_reply(event.message_id)
                .add_at(&event.user_id.to_string())
                .add_text("\n")
                .add_text(msg),
            (false, false) => Message::new().add_text(msg),
        };

        event.reply(message);
    }

    pub(crate) fn is_numeric(s: &str) -> bool {
        if s.is_empty() {
            return false;
        }
        s.chars().all(|c| c.is_ascii_digit())
    }

    #[derive(Clone)]
    pub(crate) struct ShindanData {
        pub(crate) id: String,
        pub(crate) command: String,
        pub(crate) mode: String,
    }

    #[derive(Debug)]
    pub(crate) enum Mode {
        Text,
        Image,
    }

    pub(crate) async fn get_target_name_with_id(
        bot: &Arc<RuntimeBot>,
        event: &Arc<MsgEvent>,
        params: &[&str],
    ) -> Result<(String, String)> {
        let msg_segments = &event.message;
        match (
            msg_segments.get_from_index(0),
            msg_segments.get_from_index(1),
            msg_segments.get_from_index(2),
        ) {
            (Some(first), _, Some(third)) if first.type_ == "at" && third.type_ == "at" => {
                let id = third.data["qq"].as_str().unwrap();
                let name = get_member_nickname(bot, event, id).await?;
                Ok((name, id.to_string()))
            }
            (Some(first), Some(second), _) if first.type_ != "at" && second.type_ == "at" => {
                let id = second.data["qq"].as_str().unwrap();
                let name = get_member_nickname(bot, event, id).await?;
                Ok((name, id.to_string()))
            }
            _ => {
                if !params.is_empty() {
                    Ok((params.join(" "), "".to_string()))
                } else {
                    Ok((
                        event.get_sender_nickname().to_string(),
                        event.user_id.to_string(),
                    ))
                }
            }
        }
    }

    pub(crate) async fn get_member_nickname(
        bot: &Arc<RuntimeBot>,
        event: &Arc<MsgEvent>,
        user_id: &str,
    ) -> Result<String> {
        let group_id = event
            .group_id
            .ok_or_else(|| anyhow::anyhow!("No group id"))?;
        let member_info = bot
            .get_group_member_info(group_id, user_id.parse()?, true)
            .await
            .unwrap();

        Ok(member_info.data["nickname"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Invalid nickname"))?
            .to_string())
    }

    fn create_prefix_segment(config: &Config, event: &Arc<MsgEvent>) -> Option<Segment> {
        match (config.plugin.is_at, config.plugin.is_quote) {
            (true, _) => Some(Segment {
                type_: "at".to_string(),
                data: json!({ "qq": event.user_id }),
            }),
            (false, true) => Some(Segment {
                type_: "reply".to_string(),
                data: json!({ "id": event.message_id }),
            }),
            _ => None,
        }
    }

    fn create_message(segments: &Segments, prefix: Option<Segment>) -> Message {
        let add = if prefix.is_some() { 2 } else { 0 };
        let mut result = Vec::with_capacity(segments.0.len() + add);

        if let Some(segment) = prefix {
            result.push(segment);
            result.push(Segment {
                type_: "text".to_string(),
                data: json!({ "text": "\n" }),
            });
        }

        result.extend(
            segments
                .0
                .iter()
                .map(|segment| Segment::new(&segment.type_, segment.data.clone())),
        );

        Message::from(result)
    }

    pub(crate) async fn process_text_mode(
        event: &Arc<MsgEvent>,
        data: &Arc<Data>,
        client: &Arc<ShindanClient>,
        shindan: &ShindanData,
        name: &str,
        is_random: bool,
    ) {
        let segments = match client.get_segments(&shindan.id, name).await {
            Ok(segments) => segments,
            Err(err) => {
                log::error!("[shindan-maker]: Error: {err:?}");
                build_and_send_message(event, data, SHINDAN_ERROR_MSG);
                return;
            }
        };

        if data.config.plugin.random_return_command && is_random {
            send_command_message(event, data, &shindan.command);
        }

        let prefix_segment = create_prefix_segment(&data.config, event);
        let msg = create_message(&segments, prefix_segment);

        event.reply(msg);
    }

    pub(crate) async fn process_image_mode(
        event: &Arc<MsgEvent>,
        data: &Arc<Data>,
        client: &Arc<ShindanClient>,
        shindan: &ShindanData,
        name: &str,
        is_random: bool,
    ) {
        let html = match client.get_html_str(&shindan.id, name).await {
            Ok(html) => html,
            Err(err) => {
                log::error!("[shindan-maker]: Error: {err:?}");
                build_and_send_message(event, data, SHINDAN_ERROR_MSG);
                return;
            }
        };

        if data.config.plugin.random_return_command && is_random {
            send_command_message(event, data, &shindan.command);
        }

        let browser = Browser::instance().await;
        let base64 = {
            if html.contains("chart.js") || html.contains("chartType") {
                capture_html_with_chart(&html, &browser).await
            } else {
                browser.capture_html(&html, "#title_and_result").await
            }
        };

        let base64 = match base64 {
            Ok(base64) => base64,
            Err(err) => {
                log::error!("[shindan-maker]: Error: {err:?}");
                build_and_send_message(event, data, SHINDAN_ERROR_MSG);
                return;
            }
        };

        let msg = Message::new().add_image(&format!("base64://{base64}"));
        event.reply(msg);
    }

    async fn capture_html_with_chart(html: &str, browser: &Browser) -> Result<String> {
        let tab = browser.new_tab().await?;
        tab.set_content(html).await?;
        time::sleep(time::Duration::from_secs(2)).await;
        let element = tab.find_element("#title_and_result").await?;
        let base64 = element.screenshot().await?;
        tab.close().await?;
        Ok(base64)
    }

    fn send_command_message(event: &Arc<MsgEvent>, data: &Arc<Data>, command: &str) {
        let msg = command.to_string();
        build_and_send_message(event, data, &msg);
    }

    pub(crate) fn should_show_help(params: &[&str], required_count: usize) -> bool {
        params.len() < required_count || matches!(params.first(), Some(&"-h") | Some(&"--help"))
    }

    #[derive(Clone)]
    pub enum ShindanCommandType {
        Random,
        Specific(String),
    }

    pub(crate) async fn process_shindan_command(
        bot: &Arc<RuntimeBot>,
        event: &Arc<MsgEvent>,
        data: &Arc<Data>,
        client: &Arc<ShindanClient>,
        params: &[&str],
        command: &str,
        command_type: ShindanCommandType,
    ) {
        if should_show_help(params, 0) {
            show_help_message(event, data, command);
            return;
        }

        let filtered_params = filter_mode_params(params);
        let (name, _id) = get_target_name_with_id(bot, event, &filtered_params)
            .await
            .unwrap();

        let shindan_info = match get_shindan_info(data, command_type.clone()) {
            Some(info) => info,
            None => {
                let error_msg = if matches!(command_type, ShindanCommandType::Random) {
                    "神断失败: 没有可用的神断，请先添加。"
                } else {
                    "神断失败: 找不到指定的神断命令。"
                };
                build_and_send_message(event, data, error_msg);
                return;
            }
        };

        let mode = match params.iter().find(|&&p| p == "-t" || p == "-i") {
            Some(&"-t") => Mode::Text,
            Some(&"-i") => Mode::Image,
            _ => match shindan_info.mode.as_str() {
                "text" => Mode::Text,
                _ => Mode::Image,
            },
        };

        update_statistics(data, event, &name, &shindan_info);

        let is_random = matches!(command_type, ShindanCommandType::Random);

        process_shindan_result(event, data, client, &shindan_info, &name, mode, is_random).await;
    }

    fn show_help_message(event: &Arc<MsgEvent>, data: &Arc<Data>, command: &str) {
        let help_msg = format!("{command} [名字] [-t/-i]\n示例: {command} 九条可憐 -i");
        build_and_send_message(event, data, &help_msg);
    }

    fn filter_mode_params<'a>(params: &'a [&str]) -> Vec<&'a str> {
        params
            .iter()
            .filter(|&&p| p != "-t" && p != "-i")
            .copied()
            .collect()
    }

    fn get_shindan_info(data: &Arc<Data>, command_type: ShindanCommandType) -> Option<ShindanData> {
        let guard = data.shindans.read().unwrap();
        match command_type {
            ShindanCommandType::Random => {
                guard
                    .shindan
                    .choose(&mut rand::thread_rng())
                    .map(|s| ShindanData {
                        id: s.id.clone(),
                        command: s.command.clone(),
                        mode: s.mode.clone(),
                    })
            }
            ShindanCommandType::Specific(cmd) => guard
                .shindan
                .iter()
                .find(|s| s.command == cmd)
                .map(|s| ShindanData {
                    id: s.id.clone(),
                    command: s.command.clone(),
                    mode: s.mode.clone(),
                }),
        }
    }

    fn update_statistics(
        data: &Arc<Data>,
        event: &Arc<MsgEvent>,
        name: &str,
        shindan_info: &ShindanData,
    ) {
        {
            let mut user_data = data.user_data.write().unwrap();
            match user_data
                .user
                .iter_mut()
                .find(|u| u.id.parse::<i64>().unwrap() == event.user_id)
            {
                Some(user) => user.count += 1,
                None => user_data.user.push(UserData {
                    id: event.user_id.to_string(),
                    name: name.to_string(),
                    count: 1,
                }),
            }
        }

        {
            let mut shindan = data.shindans.write().unwrap();
            if let Some(s) = shindan.shindan.iter_mut().find(|s| s.id == shindan_info.id) {
                s.count += 1;
            }
        }
    }

    async fn process_shindan_result(
        event: &Arc<MsgEvent>,
        data: &Arc<Data>,
        client: &Arc<ShindanClient>,
        shindan_info: &ShindanData,
        name: &str,
        mode: Mode,
        is_random: bool,
    ) {
        match mode {
            Mode::Text => {
                process_text_mode(event, data, client, shindan_info, name, is_random).await
            }
            Mode::Image => {
                process_image_mode(event, data, client, shindan_info, name, is_random).await
            }
        }
    }

    pub(crate) async fn update_user_name(event: &Arc<MsgEvent>, data: &Arc<Data>) {
        let now_name = event.get_sender_nickname();
        let mut user_data = data.user_data.write().unwrap();
        if let Some(u) = user_data
            .user
            .iter_mut()
            .find(|u| u.id.parse::<i64>().unwrap() == event.user_id && u.name != now_name)
        {
            u.name = now_name.to_string();
        }
    }

    pub(crate) fn parse_count(data: &Arc<Data>, params: &[&str]) -> u32 {
        params
            .first()
            .and_then(|param| param.parse::<u32>().ok())
            .unwrap_or(10)
            .min(data.config.plugin.rank_max)
    }
}
// --- END OF MODULE: plugin_utils ---

// --- START OF MODULE: commands ---
mod commands {
    use std::cmp;
    use std::fmt::Write;
    use std::sync::Arc;

    use kovi::bot::message::Segment;
    use kovi::serde_json::json;
    use kovi::{Message, MsgEvent, RuntimeBot};
    use shindan_maker::ShindanClient;

    use super::data::Data;
    use super::plugin_utils;
    use super::plugin_utils::ShindanCommandType;

    pub(crate) fn plugin_commands(event: &Arc<MsgEvent>, data: &Arc<Data>) {
        let prefix = data
            .config
            .plugin
            .prefixes
            .first()
            .map(|s| s.as_str())
            .unwrap_or("");

        // 收集所有指令名称（取别名列表中的第一个）
        let command_names: Vec<String> = data
            .commands
            .command
            .iter()
            .map(|cmd| cmd.commands.first().unwrap().to_string())
            .collect();

        let mut msg = String::new();

        // 双栏排版
        for chunk in command_names.chunks(2) {
            if let Some(first) = chunk.first() {
                write!(msg, "{}{}", prefix, first).unwrap();
            }
            if let Some(second) = chunk.get(1) {
                write!(msg, " {}{}", prefix, second).unwrap();
            }
            msg.push('\n');
        }

        // 帮助提示
        if let Some(example_cmd) = command_names.get(1) {
            write!(msg, "\n{0}{1} -h / {0}{1} --help", prefix, example_cmd).unwrap();
        }

        plugin_utils::build_and_send_message(event, data, msg.trim());
    }

    pub(crate) async fn add_shindan_command(
        event: &Arc<MsgEvent>,
        data: &Arc<Data>,
        client: &Arc<ShindanClient>,
        params: &[&str],
        command: &str,
    ) {
        // 简洁帮助
        let help_msg = format!("{command} <命令> <ID> [模式]\n示例: {command} 声优 12345 image");

        if plugin_utils::should_show_help(params, 2) {
            plugin_utils::build_and_send_message(event, data, &help_msg);
            return;
        }

        let shindan_command = params[0];
        let id = params[1];
        if !plugin_utils::is_numeric(id) {
            plugin_utils::build_and_send_message(event, data, &help_msg);
            return;
        }

        if let Some(s) = data
            .shindans
            .read()
            .unwrap()
            .shindan
            .iter()
            .find(|s| s.command == shindan_command)
        {
            let error_msg = format!("添加失败: 命令重复\n现有: {} (ID: {})", s.command, s.id);
            plugin_utils::build_and_send_message(event, data, &error_msg);
            return;
        }

        if let Some(s) = data
            .shindans
            .read()
            .unwrap()
            .shindan
            .iter()
            .find(|s| s.id == id)
        {
            let error_msg = format!("添加失败: ID 重复\n现有: {} (ID: {})", s.command, s.id);
            plugin_utils::build_and_send_message(event, data, &error_msg);
            return;
        }

        let mode = params.get(2).unwrap_or(&"image");

        if *mode != "text" && *mode != "image" {
            plugin_utils::build_and_send_message(
                event,
                data,
                "添加失败: 模式错误 (仅支持 text / image)",
            );
            return;
        }

        let (title, description) = match client.get_title_with_description(id).await {
            Ok((title, description)) => (title, description),
            Err(_) => {
                plugin_utils::build_and_send_message(event, data, "添加失败: 网络波动或无效ID");
                return;
            }
        };

        data.add_shindan(id, &title, &description, shindan_command, mode);

        let success_msg = format!("添加成功: {}\nID: {}\n模式: {}", title, id, mode);
        plugin_utils::build_and_send_message(event, data, &success_msg);
    }

    pub(crate) async fn delete_shindan_command(
        event: &Arc<MsgEvent>,
        data: &Arc<Data>,
        params: &[&str],
        command: &str,
    ) {
        let help_msg = format!("{command} <命令>\n示例: {command} 声优");

        if plugin_utils::should_show_help(params, 1) {
            plugin_utils::build_and_send_message(event, data, &help_msg);
            return;
        }

        let shindan_command = params[0];
        let deleted_info = {
            let shindan_lock = data.shindans.read().unwrap();
            shindan_lock
                .shindan
                .iter()
                .find(|s| s.command == shindan_command)
                .cloned()
        };

        if let Some(shindan) = deleted_info {
            {
                data.shindans
                    .write()
                    .unwrap()
                    .shindan
                    .retain(|s| s.command != shindan_command);
            }
            let msg = format!("删除成功: {}\nID: {}", shindan.title, shindan.id);
            plugin_utils::build_and_send_message(event, data, &msg);
        } else {
            plugin_utils::build_and_send_message(event, data, "删除失败: 神断不存在");
        }
    }

    pub(crate) async fn random_shindan_command(
        bot: &Arc<RuntimeBot>,
        event: &Arc<MsgEvent>,
        data: &Arc<Data>,
        client: &Arc<ShindanClient>,
        params: &[&str],
        command: &str,
    ) {
        plugin_utils::process_shindan_command(
            bot,
            event,
            data,
            client,
            params,
            command,
            ShindanCommandType::Random,
        )
        .await;
    }

    pub(crate) async fn specific_shindan_command(
        bot: &Arc<RuntimeBot>,
        event: &Arc<MsgEvent>,
        data: &Arc<Data>,
        client: &Arc<ShindanClient>,
        params: &[&str],
        command: &str,
    ) {
        plugin_utils::process_shindan_command(
            bot,
            event,
            data,
            client,
            params,
            command,
            ShindanCommandType::Specific(command.to_string()),
        )
        .await;
    }

    pub(crate) async fn shindan_command_list(event: &Arc<MsgEvent>, data: &Arc<Data>) {
        const PAGE_SIZE: usize = 100;
        let shindans = data.shindans.read().unwrap();
        let commands: Vec<String> = shindans.shindan.iter().map(|s| s.command.clone()).collect();

        if commands.is_empty() {
            plugin_utils::build_and_send_message(event, data, "列表为空");
            return;
        }

        let total_pages = commands.len().div_ceil(PAGE_SIZE);
        let mut forward = Vec::new();

        for page in 0..total_pages {
            let start = page * PAGE_SIZE;
            let end = cmp::min(start + PAGE_SIZE, commands.len());
            let page_commands = &commands[start..end];
            let message = format!(
                "页 {}/{}\n{}",
                page + 1,
                total_pages,
                page_commands.join(" ")
            );

            forward.push(Segment::new(
                "node",
                json!({
                    "content": [{"type": "text", "data": {"text": message}}]
                }),
            ));
        }

        event.reply(Message::from(forward));
    }

    pub(crate) async fn set_shindan_mode(
        event: &Arc<MsgEvent>,
        data: &Arc<Data>,
        params: &[&str],
        command: &str,
    ) {
        let help_msg = format!("{command} <命令> <模式>\n示例: {command} 声优 text");

        if plugin_utils::should_show_help(params, 2) {
            plugin_utils::build_and_send_message(event, data, &help_msg);
            return;
        }

        let shindan_command = params[0];
        let mode = params[1];

        if mode != "text" && mode != "image" {
            plugin_utils::build_and_send_message(event, data, "设置失败: 模式错误");
            return;
        }

        let mut shindans = data.shindans.write().unwrap();
        if let Some(s) = shindans
            .shindan
            .iter_mut()
            .find(|s| s.command == shindan_command)
        {
            s.mode = mode.to_string();
            let msg = format!("设置成功: {} -> {}", shindan_command, mode);
            plugin_utils::build_and_send_message(event, data, &msg);
        } else {
            plugin_utils::build_and_send_message(event, data, "设置失败: 神断不存在");
        }
    }

    pub(crate) async fn modify_shindan_command(
        event: &Arc<MsgEvent>,
        data: &Arc<Data>,
        params: &[&str],
        command: &str,
    ) {
        let help_msg = format!("{command} <旧命令> <新命令>\n示例: {command} 声优 配音");

        if plugin_utils::should_show_help(params, 2) {
            plugin_utils::build_and_send_message(event, data, &help_msg);
            return;
        }

        let old_cmd = params[0];
        let new_cmd = params[1];

        let mut shindans = data.shindans.write().unwrap();
        if let Some(s) = shindans.shindan.iter_mut().find(|s| s.command == old_cmd) {
            s.command = new_cmd.to_string();
            let msg = format!("修改成功: {} -> {}", old_cmd, new_cmd);
            plugin_utils::build_and_send_message(event, data, &msg);
        } else {
            plugin_utils::build_and_send_message(event, data, "修改失败: 神断不存在");
        }
    }

    pub(crate) async fn view_user_shindan_count(
        bot: &Arc<RuntimeBot>,
        event: &Arc<MsgEvent>,
        data: &Arc<Data>,
        params: &[&str],
        command: &str,
    ) {
        let help_msg = format!("{command} [@用户]");

        if plugin_utils::should_show_help(params, 0) {
            plugin_utils::build_and_send_message(event, data, &help_msg);
            return;
        }

        let (_name, id) = plugin_utils::get_target_name_with_id(bot, event, params)
            .await
            .unwrap();
        let user_data_lock = data.user_data.read().unwrap();

        if let Some(u) = user_data_lock.user.iter().find(|u| u.id == id) {
            let msg = format!("{}\n神断次数: {}", u.name, u.count);
            plugin_utils::build_and_send_message(event, data, &msg);
        } else {
            plugin_utils::build_and_send_message(event, data, "用户无记录");
        }
    }

    pub(crate) async fn user_shindan_count_rank(
        event: &Arc<MsgEvent>,
        data: &Arc<Data>,
        params: &[&str],
        command: &str,
    ) {
        let help_msg = format!("{command} [数量]");
        if plugin_utils::should_show_help(params, 0) {
            plugin_utils::build_and_send_message(event, data, &help_msg);
            return;
        }

        let count = plugin_utils::parse_count(data, params) as usize;
        let user_data_lock = data.user_data.read().unwrap();
        let mut user_data = user_data_lock.user.clone();
        user_data.sort_by(|a, b| b.count.cmp(&a.count));

        if user_data.is_empty() {
            plugin_utils::build_and_send_message(event, data, "无用户数据");
            return;
        }

        let end = cmp::min(count, user_data.len());
        let msg = user_data[..end]
            .iter()
            .map(|u| format!("{} : {}", u.name, u.count))
            .collect::<Vec<_>>()
            .join("\n");

        plugin_utils::build_and_send_message(event, data, &msg);
    }

    pub(crate) async fn view_shindan_info(
        event: &Arc<MsgEvent>,
        data: &Arc<Data>,
        params: &[&str],
        command: &str,
    ) {
        let help_msg = format!("{command} <命令/ID>");
        if plugin_utils::should_show_help(params, 1) {
            plugin_utils::build_and_send_message(event, data, &help_msg);
            return;
        }

        let target = params[0];
        let shindan_lock = data.shindans.read().unwrap();
        if let Some(s) = shindan_lock
            .shindan
            .iter()
            .find(|s| s.command == target || s.id == target)
        {
            let msg = format!(
                "标题: {}\nID: {}\n命令: {}\n模式: {}\n描述: {}",
                s.title, s.id, s.command, s.mode, s.description
            );
            plugin_utils::build_and_send_message(event, data, &msg);
        } else {
            plugin_utils::build_and_send_message(event, data, "神断不存在");
        }
    }

    pub(crate) async fn shindan_count_rank(
        event: &Arc<MsgEvent>,
        data: &Arc<Data>,
        params: &[&str],
        command: &str,
    ) {
        let help_msg = format!("{command} [数量]");
        if plugin_utils::should_show_help(params, 0) {
            plugin_utils::build_and_send_message(event, data, &help_msg);
            return;
        }

        let count = plugin_utils::parse_count(data, params) as usize;
        let shindan_lock = data.shindans.read().unwrap();
        let mut items: Vec<_> = shindan_lock.shindan.iter().collect();
        items.sort_by(|a, b| b.count.cmp(&a.count));

        if items.is_empty() {
            plugin_utils::build_and_send_message(event, data, "无神断数据");
            return;
        }

        let end = cmp::min(count, items.len());
        let msg = items[..end]
            .iter()
            .map(|s| format!("{} : {}", s.command, s.count))
            .collect::<Vec<_>>()
            .join("\n");

        plugin_utils::build_and_send_message(event, data, &msg);
    }

    pub(crate) async fn fuzzy_search_shindan_command(
        event: &Arc<MsgEvent>,
        data: &Arc<Data>,
        params: &[&str],
        command: &str,
    ) {
        let help_msg = format!("{command} <关键词> [数量]");
        if plugin_utils::should_show_help(params, 1) {
            plugin_utils::build_and_send_message(event, data, &help_msg);
            return;
        }

        let keyword = params[0];
        let count = plugin_utils::parse_count(data, params) as usize;

        let shindan_lock = data.shindans.read().unwrap();
        let matches: Vec<_> = shindan_lock
            .shindan
            .iter()
            .filter(|s| s.command.contains(keyword))
            .collect();

        if matches.is_empty() {
            plugin_utils::build_and_send_message(event, data, "未找到相关神断");
            return;
        }

        let total = matches.len();
        let end = cmp::min(count, total);
        let mut msg = matches[..end]
            .iter()
            .enumerate()
            .map(|(i, s)| format!("{}. {}", i + 1, s.command))
            .collect::<Vec<_>>()
            .join("\n");

        if end < total {
            msg.push_str(&format!("\n... 共 {} 项", total));
        }

        plugin_utils::build_and_send_message(event, data, &msg);
    }
}
// --- END OF MODULE: commands ---

// --- START OF MAIN PLUGIN LOGIC ---

use cdp_html_shot::Browser;
use kovi::PluginBuilder;
use shindan_maker::ShindanClient;
use std::sync::Arc;

use self::data::Data;

#[kovi::plugin]
async fn main() {
    // 初始化全局浏览器实例
    Browser::instance().await;

    let data = Arc::new(Data::new());
    let bot = PluginBuilder::get_runtime_bot();
    let client = Arc::new(ShindanClient::new(data.config.plugin.domain.parse().unwrap()).unwrap());

    PluginBuilder::on_msg({
        let data = Arc::clone(&data);
        let bot = Arc::clone(&bot);
        let client = Arc::clone(&client);
        move |event| {
            let data = Arc::clone(&data);
            let bot = Arc::clone(&bot);
            let client = Arc::clone(&client);
            async move {
                // 仅 @机器人 时响应
                if data.config.plugin.only_at {
                    let message = &event.message;
                    let segment = message.get_from_index(0).unwrap();
                    if segment.type_ != "at"
                        || segment.data["qq"].as_str().unwrap().parse::<i64>().unwrap()
                            != event.self_id
                    {
                        return;
                    }
                }

                // 无意义消息过滤
                let text = match event.borrow_text() {
                    None => return,
                    Some(text) => text,
                };

                // 频道过滤
                if !plugin_utils::should_process_group(
                    event.group_id.as_ref(),
                    &data.config.channel.white,
                    &data.config.channel.black,
                ) {
                    return;
                }

                // 指令解析
                if let Some((cmd, params)) =
                    plugin_utils::parse_command(text, &data.config.plugin.prefixes)
                {
                    // 更新用户名
                    plugin_utils::update_user_name(&event, &data).await;

                    // 插件指令
                    if let Some(command) = data.commands.get_command(cmd) {
                        let function = command.function.as_str();
                        match function {
                            "插件指令列表" => commands::plugin_commands(&event, &data),
                            "添加神断命令" => {
                                commands::add_shindan_command(&event, &data, &client, &params, cmd)
                                    .await
                            }
                            "删除神断命令" => {
                                commands::delete_shindan_command(&event, &data, &params, cmd).await
                            }
                            "随机神断命令" => {
                                commands::random_shindan_command(
                                    &bot, &event, &data, &client, &params, cmd,
                                )
                                .await
                            }
                            "神断命令列表" => {
                                commands::shindan_command_list(&event, &data).await
                            }
                            "设置神断模式" => {
                                commands::set_shindan_mode(&event, &data, &params, cmd).await
                            }
                            "修改神断命令" => {
                                commands::modify_shindan_command(&event, &data, &params, cmd).await
                            }
                            "查看用户神断次数" => {
                                commands::view_user_shindan_count(&bot, &event, &data, &params, cmd)
                                    .await
                            }
                            "用户神断次数排行榜" => {
                                commands::user_shindan_count_rank(&event, &data, &params, cmd).await
                            }
                            "查看神断信息" => {
                                commands::view_shindan_info(&event, &data, &params, cmd).await
                            }
                            "神断被触发次数排行榜" => {
                                commands::shindan_count_rank(&event, &data, &params, cmd).await
                            }
                            "模糊查找神断命令" => {
                                commands::fuzzy_search_shindan_command(&event, &data, &params, cmd)
                                    .await
                            }
                            _ => {}
                        }
                    }

                    // 神断命令
                    if data
                        .shindans
                        .read()
                        .unwrap()
                        .shindan
                        .iter()
                        .any(|s| s.command == cmd)
                    {
                        commands::specific_shindan_command(
                            &bot, &event, &data, &client, &params, cmd,
                        )
                        .await;
                    }
                }
            }
        }
    });

    PluginBuilder::drop({
        let data = Arc::clone(&data);

        move || {
            let data = Arc::clone(&data);
            async move {
                // 关闭全局浏览器实例
                Browser::instance().await.close_async().await.unwrap();

                // 保存数据
                data.save();
            }
        }
    });
}
// --- END OF MAIN PLUGIN LOGIC ---
