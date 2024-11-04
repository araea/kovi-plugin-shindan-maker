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
    pub(crate) is_reply: bool,
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

#[derive(Debug, Serialize, Deserialize)]
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
