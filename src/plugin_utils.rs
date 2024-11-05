use std::sync::Arc;
use anyhow::Result;
use kovi::serde_json::json;
use cdp_html_shot::Browser;
use rand::prelude::SliceRandom;
use kovi::bot::message::Segment;
use shindan_maker::{Segments, ShindanClient};
use kovi::{AllMsgEvent, Message, RuntimeBot};
use kovi::tokio::time;
use crate::data::Data;
use crate::types::{Config, UserData};

const SHINDAN_ERROR_MSG: &str = "神断失败！\n\n[原因]\n神累了";

pub(crate) fn should_process_group(
    group_id: Option<&i64>,
    white_list: &[String],
    black_list: &[String],
) -> bool {
    let group_id = match group_id {
        Some(id) => id.to_string(),
        None => return true
    };

    if black_list.contains(&group_id) {
        return false;
    }

    if !white_list.is_empty() && !white_list.contains(&group_id) {
        return false;
    }

    true
}

pub(crate) fn parse_command<'a>(text: &'a str, prefixes: &[String]) -> Option<(&'a str, Vec<&'a str>)> {
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

        command = sorted_prefixes.iter()
            .find(|&p| command.starts_with(p))
            .map(|p| &command[p.len()..])?;

        Some((command, words))
    }
}

pub(crate) fn build_and_send_message(event: &Arc<AllMsgEvent>, data: &Arc<Data>, msg: &str) {
    let message = match (data.config.plugin.is_at, data.config.plugin.is_quote) {
        (true, false) => Message::new()
            .add_at(&event.user_id.to_string())
            .add_text("\n")
            .add_text(msg),
        (false, true) => Message::new()
            .add_reply(event.message_id)
            .add_text(msg),
        (true, true) => Message::new()
            .add_reply(event.message_id)
            .add_at(&event.user_id.to_string())
            .add_text("\n")
            .add_text(msg),
        (false, false) => Message::new()
            .add_text(msg),
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
}

#[derive(Debug)]
pub(crate) enum Mode {
    Text,
    Image,
}

pub(crate) async fn get_target_name_with_id(bot: &Arc<RuntimeBot>, event: &Arc<AllMsgEvent>, params: &[&str]) -> Result<(String, String)> {
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
            if params.first().is_some() {
                Ok((params.join(" "), "".to_string()))
            } else {
                Ok((event.get_sender_nickname().to_string(), event.user_id.to_string()))
            }
        }
    }
}

pub(crate) async fn get_member_nickname(bot: &Arc<RuntimeBot>, event: &Arc<AllMsgEvent>, user_id: &str) -> Result<String> {
    let group_id = event.group_id.ok_or_else(|| anyhow::anyhow!("No group id"))?;
    let member_info = bot
        .get_group_member_info(group_id, user_id.parse()?, true)
        .await.unwrap();

    Ok(member_info.data["nickname"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("Invalid nickname"))?
        .to_string())
}

fn create_prefix_segment(config: &Config, event: &Arc<AllMsgEvent>) -> Option<Segment> {
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
            .map(|segment| Segment::new(&segment.type_, segment.data.clone()))
    );

    Message::from(result)
}

pub(crate) async fn process_text_mode(
    event: &Arc<AllMsgEvent>,
    data: &Arc<Data>,
    client: &Arc<ShindanClient>,
    shindan: &ShindanData,
    name: &str,
    is_random: bool,
) {
    let segments = match client.get_segments(&shindan.id, name).await {
        Ok(segments) => segments,
        Err(_) => {
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
    event: &Arc<AllMsgEvent>,
    data: &Arc<Data>,
    client: &Arc<ShindanClient>,
    shindan: &ShindanData,
    name: &str,
    is_random: bool,
) {
    let html = match client.get_html_str(&shindan.id, name).await {
        Ok(html) => html,
        Err(_) => {
            build_and_send_message(event, data, SHINDAN_ERROR_MSG);
            return;
        }
    };

    if data.config.plugin.random_return_command && is_random {
        send_command_message(event, data, &shindan.command);
    }

    let browser = Browser::instance().await;
    let base64 = {
        if html.contains("chart.js") {
            capture_html_with_chart(&html, &browser).await
        } else {
            browser.capture_html(&html, "#title_and_result").await
        }
    };

    let base64 = match base64 {
        Ok(base64) => base64,
        Err(_) => {
            build_and_send_message(event, data, SHINDAN_ERROR_MSG);
            return;
        }
    };

    let msg = Message::new().add_image(&format!("base64://{}", base64));
    event.reply(msg);
}

async fn capture_html_with_chart(html: &str, browser: &Arc<Browser>) -> Result<String> {
    let tab = browser.new_tab().await?;
    tab.set_content(html).await?;
    time::sleep(time::Duration::from_secs(2)).await;
    let element = tab.find_element("#title_and_result").await?;
    let base64 = element.screenshot().await?;
    tab.close().await?;
    Ok(base64)
}

pub(crate) fn send_command_message(event: &Arc<AllMsgEvent>, data: &Arc<Data>, command: &str) {
    let msg = format!("[神断命令]\n{command}");
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
    event: &Arc<AllMsgEvent>,
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

    let mode = {
        if params.iter().any(|p| *p == "-t") {
            Mode::Text
        } else if params.iter().any(|p| *p == "-i") {
            Mode::Image
        } else {
            let shindan_data = data.shindans.read().unwrap();

            shindan_data
                .shindan
                .iter()
                .find(|s| s.command == command)
                .map_or(Mode::Image, |s| match s.mode.as_str() {
                    "image" => Mode::Image,
                    "text" => Mode::Text,
                    _ => Mode::Image,
                })
        }
    };

    let filtered_params = filter_mode_params(params);
    let (name, _id) = get_target_name_with_id(bot, event, &filtered_params).await.unwrap();

    let shindan_info = get_shindan_info(data, command_type.clone());

    update_statistics(data, event, &name, &shindan_info);

    let is_random = matches!(command_type, ShindanCommandType::Random);

    process_shindan_result(event, data, client, &shindan_info, &name, mode, is_random).await;
}

fn show_help_message(event: &Arc<AllMsgEvent>, data: &Arc<Data>, command: &str) {
    let help_msg = format!(
        "[指令]\n{command}\n\n[功能]\n神断命令\n\n[参数]\n\n1. 名字（可选）\n2. 模式 -t / -i（可选）\n\n[示例]\n\n1. {command}\n2. {command} 九条可憐\n3. {command} 九条可憐 -t\n4. {command} 九条可憐 -i\n5. {command} @九条可憐"
    );
    build_and_send_message(event, data, &help_msg);
}

fn filter_mode_params<'a>(params: &'a [&str]) -> Vec<&'a str> {
    params.iter()
        .filter(|&&p| p != "-t" && p != "-i")
        .copied()
        .collect()
}

fn get_shindan_info(data: &Arc<Data>, command_type: ShindanCommandType) -> ShindanData {
    let guard = data.shindans.read().unwrap();
    match command_type {
        ShindanCommandType::Random => {
            let chosen = guard.shindan
                .choose(&mut rand::thread_rng())
                .unwrap();
            ShindanData {
                id: chosen.id.clone(),
                command: chosen.command.clone(),
            }
        }
        ShindanCommandType::Specific(cmd) => {
            guard.shindan
                .iter()
                .find(|s| s.command == cmd)
                .map(|s| ShindanData {
                    id: s.id.clone(),
                    command: s.command.clone(),
                })
                .unwrap()
        }
    }
}

fn update_statistics(data: &Arc<Data>, event: &Arc<AllMsgEvent>, name: &str, shindan_info: &ShindanData) {
    {
        let mut user_data = data.user_data.write().unwrap();
        match user_data.user.iter_mut().find(|u| u.id.parse::<i64>().unwrap() == event.user_id) {
            Some(user) => user.count += 1,
            None => user_data.user.push(UserData {
                id: event.user_id.clone().to_string(),
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
    event: &Arc<AllMsgEvent>,
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

pub(crate) async fn update_user_name(event: &Arc<AllMsgEvent>, data: &Arc<Data>) {
    let now_name = event.get_sender_nickname();
    let mut user_data = data.user_data.write().unwrap();
    if let Some(u) = user_data.user.iter_mut()
        .find(|u| u.id.parse::<i64>().unwrap() == event.user_id && u.name != now_name) {
        u.name = now_name.to_string();
    }
}