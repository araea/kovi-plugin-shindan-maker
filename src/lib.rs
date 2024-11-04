#![doc = include_str!("../README.md")]

mod data;
mod types;
mod commands;
mod init_data;
mod plugin_utils;

use std::sync::Arc;
use kovi::PluginBuilder;
use cdp_html_shot::Browser;
use shindan_maker::ShindanClient;

use crate::data::Data;

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
                    if segment.type_ != "at" || segment.data["qq"].as_str().unwrap().parse::<i64>().unwrap() != event.self_id {
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
                if let Some((cmd, params)) = plugin_utils::parse_command(text, &data.config.plugin.prefixes) {
                    // 更新用户名
                    plugin_utils::update_user_name(&event, &data).await;

                    // 插件指令
                    if let Some(command) = data.commands.get_command(cmd) {
                        let function = command.function.as_str();
                        match function {
                            "插件指令列表" => commands::plugin_commands(&event, &data),
                            "添加神断命令" => commands::add_shindan_command(&event, &data, &client, &params, cmd).await,
                            "删除神断命令" => commands::delete_shindan_command(&event, &data, &params, cmd).await,
                            "随机神断命令" => commands::random_shindan_command(&bot, &event, &data, &client, &params, cmd).await,
                            "神断命令列表" => commands::shindan_command_list(&event, &data).await,
                            "设置神断模式" => commands::set_shindan_mode(&event, &data, &params, cmd).await,
                            "修改神断命令" => commands::modify_shindan_command(&event, &data, &params, cmd).await,
                            "查看用户神断次数" => commands::view_user_shindan_count(&bot, &event, &data, &params, cmd).await,
                            "用户神断次数排行榜" => commands::user_shindan_count_rank(&event, &data, &params, cmd).await,
                            "查看神断信息" => commands::view_shindan_info(&event, &data, &params, cmd).await,
                            "神断被触发次数排行榜" => commands::shindan_count_rank(&event, &data, &params, cmd).await,
                            "模糊查找神断命令" => commands::fuzzy_search_shindan_command(&event, &data, &params, cmd).await,
                            _ => {}
                        }
                    }

                    // 神断命令
                    if data.shindans.read().unwrap().shindan.iter().any(|s| s.command == cmd) {
                        commands::specific_shindan_command(&bot, &event, &data, &client, &params, cmd).await;
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
                Browser::close_instance();

                // 保存数据
                data.save();
            }
        }
    }
    );
}


