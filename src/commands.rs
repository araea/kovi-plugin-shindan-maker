use crate::data::Data;
use crate::plugin_utils;
use crate::plugin_utils::ShindanCommandType;
use kovi::bot::message::Segment;
use kovi::serde_json::json;
use kovi::{AllMsgEvent, Message, RuntimeBot};
use shindan_maker::ShindanClient;
use std::cmp;
use std::sync::Arc;

pub(crate) fn plugin_commands(event: &Arc<AllMsgEvent>, data: &Arc<Data>) {
    let commands = &data
        .commands
        .command
        .iter()
        .enumerate()
        .map(|(index, command)| format!("{}. {}", index + 1, command.commands.join(" / ")))
        .collect::<Vec<String>>()
        .join("\n");

    let mut str = String::new();
    let prefixes_str = data.config.plugin.prefixes.join("\n");
    if !prefixes_str.is_empty() {
        str = format!("\n\n[指令.前缀]\n{prefixes_str}");
    }

    let msg = format!(
        "[指令]

{commands}{str}

[帮助]
-h / --help 查看帮助

[帮助.示例]

1. {0} -h
2. {0} --help",
        &data.commands.command[1].commands[0]
    );

    plugin_utils::build_and_send_message(event, data, &msg);
}

pub(crate) async fn add_shindan_command(
    event: &Arc<AllMsgEvent>,
    data: &Arc<Data>,
    client: &Arc<ShindanClient>,
    params: &[&str],
    command: &str,
) {
    let help_msg = format!(
        "[指令]
{command}

[功能]
添加神断命令

[参数]

1. 神断命令
2. 神断 ID
3. 模式 text/image（可选）

[示例]

1. {command} 声优 12345
2. {command} 声优 12345 text
3. {command} 声优 12345 image"
    );

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
        let id = &s.id;
        let mode = &s.mode;
        let shindan_title = &s.title;
        let shindan_command = &s.command;
        let description = &s.description;

        let error_msg = format!(
            "添加失败！

[原因]
神断命令重复

[重复项]
ID：{id}
标题：{shindan_title}
命令：{shindan_command}
模式：{mode}
描述：{description}"
        );
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
        let id = &s.id;
        let mode = &s.mode;
        let shindan_title = &s.title;
        let shindan_command = &s.command;
        let description = &s.description;
        let error_msg = format!(
            "添加失败！

[原因]
神断 ID 重复

[重复项]
ID：{id}
标题：{shindan_title}
命令：{shindan_command}
模式：{mode}
描述：{description}"
        );
        plugin_utils::build_and_send_message(event, data, &error_msg);
        return;
    }

    let mode = params.get(2).unwrap_or(&"image");

    if *mode != "text" && *mode != "image" {
        let error_msg = "添加失败！

[原因]
模式错误

[支持模式]
text / image"
            .to_string();
        plugin_utils::build_and_send_message(event, data, &error_msg);
        return;
    }

    let (title, description) = match client.get_title_with_description(id).await {
        Ok((title, description)) => (title, description),
        Err(_) => {
            let error_msg = "添加失败！

[原因]
网络波动 / 未知神断"
                .to_string();
            plugin_utils::build_and_send_message(event, data, &error_msg);
            return;
        }
    };

    data.add_shindan(id, &title, &description, shindan_command, mode);

    let success_msg = format!(
        "添加成功！

[新增]
ID：{id}
标题：{title}
命令：{shindan_command}
模式：{mode}
描述：{description}"
    );

    plugin_utils::build_and_send_message(event, data, &success_msg);
}

pub(crate) async fn delete_shindan_command(
    event: &Arc<AllMsgEvent>,
    data: &Arc<Data>,
    params: &[&str],
    command: &str,
) {
    let help_msg = format!(
        "[指令]
{command}

[功能]
删除神断命令

[参数]

1. 神断命令

[示例]

1. {command} 声优"
    );

    if plugin_utils::should_show_help(params, 1) {
        plugin_utils::build_and_send_message(event, data, &help_msg);
        return;
    }

    let shindan_command = params[0];

    let success_msg = {
        let shindan_lock = data.shindans.read().unwrap();
        let shindan = shindan_lock
            .shindan
            .iter()
            .find(|s| s.command == shindan_command);
        if shindan.is_none() {
            let error_msg = "删除失败！

[原因]
神断不存在"
                .to_string();
            plugin_utils::build_and_send_message(event, data, &error_msg);
            return;
        }

        let shindan = shindan.unwrap();
        let id = &shindan.id;
        let mode = &shindan.mode;
        let title = &shindan.title;
        let description = &shindan.description;
        format!(
            "删除成功！

[删除]
ID：{id}
标题：{title}
命令：{shindan_command}
模式：{mode}
描述：{description}"
        )
    };

    {
        data.shindans
            .write()
            .unwrap()
            .shindan
            .retain(|s| s.command != shindan_command);
    }

    plugin_utils::build_and_send_message(event, data, &success_msg);
}

pub(crate) async fn random_shindan_command(
    bot: &Arc<RuntimeBot>,
    event: &Arc<AllMsgEvent>,
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
    event: &Arc<AllMsgEvent>,
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

pub(crate) async fn shindan_command_list(event: &Arc<AllMsgEvent>, data: &Arc<Data>) {
    const PAGE_SIZE: usize = 100;

    let shindans = data.shindans.read().unwrap();

    let commands: Vec<String> = shindans.shindan.iter().map(|s| s.command.clone()).collect();

    let total_commands = commands.len();
    let total_pages = (total_commands + PAGE_SIZE - 1) / PAGE_SIZE;

    if total_commands == 0 {
        let msg = "获取失败！\n\n[原因]\n无神断命令".to_string();
        plugin_utils::build_and_send_message(event, data, &msg);
        return;
    }

    let mut forward = Vec::new();

    for page in 0..total_pages {
        let start = page * PAGE_SIZE;
        let end = cmp::min(start + PAGE_SIZE, total_commands);

        let page_commands = &commands[start..end];
        let message = format!("[{}-{}]\n{}", start + 1, end, page_commands.join(" "));

        let segment = Segment::new(
            "node",
            json!({
                "content": [
                    {"type": "text", "data": {"text": message}}
                ]
            }),
        );
        forward.push(segment);

        // plugin_utils::build_and_send_message(event, data, &message);
    }

    event.reply(Message::from(forward));
}

pub(crate) async fn set_shindan_mode(
    event: &Arc<AllMsgEvent>,
    data: &Arc<Data>,
    params: &[&str],
    command: &str,
) {
    let help_msg = format!(
        "[指令]
{command}

[功能]
设置神断

[参数]

1. 神断命令
2. 模式 text/image

[示例]

1. {command} 声优 text
2. {command} 声优 image"
    );

    if plugin_utils::should_show_help(params, 2) {
        plugin_utils::build_and_send_message(event, data, &help_msg);
        return;
    }

    let shindan_command = params[0];
    let mode = params[1];
    if mode != "text" && mode != "image" {
        let error_msg = "设置失败！

[原因]
模式错误

[支持模式]
text / image"
            .to_string();
        plugin_utils::build_and_send_message(event, data, &error_msg);
        return;
    }

    let success_msg = {
        let shindan_lock = data.shindans.read().unwrap();
        let shindan = &mut shindan_lock
            .shindan
            .iter()
            .find(|s| s.command == shindan_command);
        if shindan.is_none() {
            let error_msg = "设置失败！

[原因]
神断不存在"
                .to_string();
            plugin_utils::build_and_send_message(event, data, &error_msg);
            return;
        }

        let shindan = shindan.unwrap();

        let id = &shindan.id;
        let title = &shindan.title;
        let description = &shindan.description;
        format!(
            "设置成功！

[设置]
ID：{id}
标题：{title}
命令：{shindan_command}
模式：{mode}
描述：{description}"
        )
    };

    {
        let mut shindans = data.shindans.write().unwrap();
        if let Some(s) = shindans
            .shindan
            .iter_mut()
            .find(|s| s.command == shindan_command)
        {
            s.mode = mode.to_string();
        }
    }

    plugin_utils::build_and_send_message(event, data, &success_msg);
}

pub(crate) async fn modify_shindan_command(
    event: &Arc<AllMsgEvent>,
    data: &Arc<Data>,
    params: &[&str],
    command: &str,
) {
    let help_msg = format!(
        "[指令]
{command}

[功能]
修改神断命令

[参数]

1. 神断命令（旧）
2. 神断命令（新）

[示例]

1. {command} 声优 配音演员
2. {command} 配音演员 配音员"
    );

    if plugin_utils::should_show_help(params, 2) {
        plugin_utils::build_and_send_message(event, data, &help_msg);
        return;
    }

    let old_shindan_command = params[0];
    let new_shindan_command = params[1];

    let success_msg = {
        let shindan_lock = data.shindans.read().unwrap();
        let shindan = shindan_lock
            .shindan
            .iter()
            .find(|s| s.command == old_shindan_command);
        if shindan.is_none() {
            let error_msg = "修改失败！

[原因]
神断不存在"
                .to_string();
            plugin_utils::build_and_send_message(event, data, &error_msg);
            return;
        }

        let shindan = shindan.unwrap();
        let id = &shindan.id;
        let title = &shindan.title;
        let description = &shindan.description;
        let mode = &shindan.mode;
        format!(
            "修改成功！

[修改]
ID：{id}
标题：{title}
命令：{old_shindan_command} -> {new_shindan_command}
模式：{mode}
描述：{description}"
        )
    };

    {
        if let Some(s) = data
            .shindans
            .write()
            .unwrap()
            .shindan
            .iter_mut()
            .find(|s| s.command == old_shindan_command)
        {
            s.command = new_shindan_command.to_string();
        }
    }

    plugin_utils::build_and_send_message(event, data, &success_msg);
}

pub(crate) async fn view_user_shindan_count(
    bot: &Arc<RuntimeBot>,
    event: &Arc<AllMsgEvent>,
    data: &Arc<Data>,
    params: &[&str],
    command: &str,
) {
    let help_msg = format!(
        "[指令]
{command}

[功能]
查看用户神断次数

[参数]

1. @用户（可选）

[示例]

1. {command}
2. {command} @九条可憐"
    );

    if plugin_utils::should_show_help(params, 0) {
        plugin_utils::build_and_send_message(event, data, &help_msg);
        return;
    }

    let (_name, id) = plugin_utils::get_target_name_with_id(bot, event, params)
        .await
        .unwrap();

    let user_data_lock = data.user_data.read().unwrap();
    let user_data = user_data_lock.user.iter().find(|u| u.id == id);
    if user_data.is_none() {
        let error_msg = "查看失败！

[原因]
用户不存在"
            .to_string();
        plugin_utils::build_and_send_message(event, data, &error_msg);
        return;
    }

    let user_data = user_data.unwrap();
    let name = &user_data.name;
    let count = user_data.count;

    let msg = format!("[用户]\n{}\n\n[神断次数]\n{} 次", name, count);
    plugin_utils::build_and_send_message(event, data, &msg);
}

pub(crate) async fn user_shindan_count_rank(
    event: &Arc<AllMsgEvent>,
    data: &Arc<Data>,
    params: &[&str],
    command: &str,
) {
    let help_msg = format!(
        "[指令]
{command}

[功能]
用户神断次数排行榜

[参数]

1. 数量（可选）

[示例]

1. {command}
2. {command} 10"
    );

    if plugin_utils::should_show_help(params, 0) {
        plugin_utils::build_and_send_message(event, data, &help_msg);
        return;
    }

    let count = plugin_utils::parse_count(data, params);

    let user_data_lock = data.user_data.read().unwrap();
    let mut user_data = user_data_lock.user.clone();
    user_data.sort_by(|a, b| b.count.cmp(&a.count));

    let total_users = user_data.len();
    let total_pages = 1;

    if total_users == 0 {
        let msg = "获取失败！\n\n[原因]\n无用户".to_string();
        plugin_utils::build_and_send_message(event, data, &msg);
        return;
    }

    for page in 0..total_pages {
        let start = (page * count) as usize;
        let end = cmp::min(start + count as usize, total_users);
        let page_users = &user_data[start..end];

        let message = format!(
            "[{}-{}]\n{}",
            start + 1,
            end,
            page_users
                .iter()
                .map(|u| format!("{}：{} 次", u.name, u.count))
                .collect::<Vec<String>>()
                .join("\n")
        );

        plugin_utils::build_and_send_message(event, data, &message);
    }
}

pub(crate) async fn view_shindan_info(
    event: &Arc<AllMsgEvent>,
    data: &Arc<Data>,
    params: &[&str],
    command: &str,
) {
    let help_msg = format!(
        "[指令]
{command}

[功能]
查看神断信息

[参数]

1. 神断命令 / 神断 ID

[示例]

1. {command} 声优
2. {command} 12345"
    );

    if plugin_utils::should_show_help(params, 1) {
        plugin_utils::build_and_send_message(event, data, &help_msg);
        return;
    }

    let shindan_command_or_shindan_id = params[0];
    let shindan_lock = data.shindans.read().unwrap();
    let shindan = shindan_lock.shindan.iter().find(|s| {
        s.command == shindan_command_or_shindan_id || s.id == shindan_command_or_shindan_id
    });
    if shindan.is_none() {
        let error_msg = "查看失败！

[原因]
神断不存在"
            .to_string();
        plugin_utils::build_and_send_message(event, data, &error_msg);
        return;
    }

    let shindan = shindan.unwrap();

    let id = &shindan.id;
    let mode = &shindan.mode;
    let title = &shindan.title;
    let description = &shindan.description;
    let shindan_command = &shindan.command;

    let msg = format!(
        "[神断信息]
ID：{id}
标题：{title}
命令：{shindan_command}
模式：{mode}
描述：{description}"
    );

    plugin_utils::build_and_send_message(event, data, &msg);
}

pub(crate) async fn shindan_count_rank(
    event: &Arc<AllMsgEvent>,
    data: &Arc<Data>,
    params: &[&str],
    command: &str,
) {
    let help_msg = format!(
        "[指令]
{command}

[功能]
神断触发次数排行榜

[参数]

1. 数量（可选）

[示例]

1. {command}
2. {command} 10"
    );

    if plugin_utils::should_show_help(params, 0) {
        plugin_utils::build_and_send_message(event, data, &help_msg);
        return;
    }

    let count = plugin_utils::parse_count(data, params);

    let mut shindan_count = {
        let shindan_lock = data.shindans.read().unwrap();
        shindan_lock
            .shindan
            .iter()
            .map(|s| (s.command.clone(), s.count))
            .collect::<Vec<(String, u32)>>()
    };

    shindan_count.sort_by(|a, b| b.1.cmp(&a.1));

    let total_shindans = shindan_count.len();
    let total_pages = 1;

    if total_shindans == 0 {
        let msg = "获取失败！\n\n[原因]\n无神断".to_string();
        plugin_utils::build_and_send_message(event, data, &msg);
        return;
    }

    for page in 0..total_pages {
        let start = (page * count) as usize;
        let end = cmp::min(start + count as usize, total_shindans);

        let page_shindans = &shindan_count[start..end];
        let message = format!(
            "[{}-{}]\n{}",
            start + 1,
            end,
            page_shindans
                .iter()
                .map(|(command, count)| format!("{}：{} 次", command, count))
                .collect::<Vec<String>>()
                .join("\n")
        );

        plugin_utils::build_and_send_message(event, data, &message);
    }
}

pub(crate) async fn fuzzy_search_shindan_command(
    event: &Arc<AllMsgEvent>,
    data: &Arc<Data>,
    params: &[&str],
    command: &str,
) {
    let help_msg = format!(
        "[指令]
{command}

[功能]
模糊查找神断命令

[参数]

1. 神断命令
2. 数量（可选）

[示例]

1. {command} 声优"
    );

    if plugin_utils::should_show_help(params, 1) {
        plugin_utils::build_and_send_message(event, data, &help_msg);
        return;
    }

    let shindan_command = params[0];

    let count = plugin_utils::parse_count(data, params);

    let shindan_lock = data.shindans.read().unwrap();
    let shindan = shindan_lock
        .shindan
        .iter()
        .filter(|s| s.command.contains(shindan_command))
        .collect::<Vec<_>>();

    let total_shindans = shindan.len();
    let total_pages = 1;

    if total_shindans == 0 {
        let msg = "获取失败！\n\n[原因]\n无神断".to_string();
        plugin_utils::build_and_send_message(event, data, &msg);
        return;
    }

    for page in 0..total_pages {
        let start = (page * count) as usize;
        let end = cmp::min(start + count as usize, total_shindans);

        let page_shindans = &shindan[start..end];

        let mut message = format!(
            "[{}-{}]\n\n{}",
            start + 1,
            end,
            page_shindans
                .iter()
                .enumerate()
                .map(|(i, s)| format!("{}. {}", i + start + 1, s.command))
                .collect::<Vec<String>>()
                .join("\n")
        );

        if end < total_shindans {
            message.push_str(
                format!("\n...\n\n[提示]\n共 {total_shindans} 项\n输入数量查看更多").as_str(),
            );
        }

        plugin_utils::build_and_send_message(event, data, &message);
    }
}
