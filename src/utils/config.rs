use serde::Deserialize;
use std::collections::HashMap;

// constants

// token
pub const DISCORD_TOKEN: &str = "discord_token";
// guild
// pub const GUILD_ID: &str = "GUILD_ID";
// pub const PRESIDENT: &str = "PRESIDENT";
// roles.private
pub const REMOVE_ROLE_ID: &str = "REMOVE_ROLE_ID";
pub const ADMIN_ROLE_ID: &str = "ADMIN_ROLE_ID";
// roles.public
// should not have things in here that could be subject to PII
// channels
// pub const DESTIN_CHANNEL: &str = "DESTIN_CHANNEL_ID";
pub const ENROLL_CHANNEL: &str = "ENROLL_CHANNEL_ID";
pub const READING_CHANNEL: &str = "READING_CHANNEL_ID";
// pub const LOGGING_CHANNEL: &str = "LOGGING_CHANNEL_ID";
pub const MEETING_CHANNEL: &str = "MEETING_CHANNEL_ID";

// config file types
#[derive(Debug, Deserialize, Clone)]
pub struct ConfigData {
    #[serde(alias = "TOKEN")]
    pub token: HashMap<String, String>,
    #[serde(alias = "GUILD")]
    pub guild: Guild,
    #[serde(alias = "ROLES")]
    pub roles: Roles,
    #[serde(alias = "CHANNELS")]
    pub channels: HashMap<String, u64>,
    #[serde(alias = "FEATURES")]
    pub features: HashMap<String, bool>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Roles {
    #[serde(alias = "PUBLIC")]
    pub public: HashMap<String, u64>,
    #[serde(alias = "PRIVATE")]
    pub private: HashMap<String, u64>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Guild {
    #[serde(alias = "MAIN")]
    pub main: GuildMain,
    #[serde(alias = "PARTNERS")]
    pub partners: HashMap<String, GuildPartners>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GuildMain {
    #[serde(alias = "GUILD_ID")]
    pub guild_id: u64,
    #[serde(alias = "PRESIDENT")]
    pub president: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GuildPartners {
    #[serde(alias = "ID")]
    pub id: u64,
    #[serde(alias = "NAME")]
    pub name: String,
    #[serde(alias = "SEND_NEWS")]
    pub send_news: bool,
    #[serde(alias = "NEWS_CHANNEL")]
    pub news_channel: u64,
}

// read and return result of the config file
pub fn get_config() -> Result<ConfigData, toml::de::Error> {
    let data = std::fs::read_to_string("config.toml").expect("Unable to find config.toml file");
    toml::from_str(&data)
}
