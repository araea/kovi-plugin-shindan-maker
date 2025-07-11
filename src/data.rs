use kovi::utils;
use kovi::{PluginBuilder, toml};
use std::path::PathBuf;
use std::sync::RwLock;

use crate::init_data::{COMMANDS_TOML, CONFIG_TOML, SHINDANS_TOML, USER_DATA_TOML};
use crate::types::{Commands, Config, Shindan, Shindans, UserDatas};

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
