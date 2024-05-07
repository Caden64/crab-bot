use std::collections::HashMap;
use serde::Deserialize;

// constants 
pub const DISCORD_TOKEN: &str = "discord_token";
pub const REMOVE_ROLE_ID: &str = "REMOVE_ROLE_ID";
pub const GUILD_ID: &str = "GUILD_ID";


#[derive(Debug, Deserialize, Clone)]
pub struct ConfigData{
    pub token: HashMap<String, String>,
    pub guild: HashMap<String, u64>,
    pub roles: Roles,
    pub channels: HashMap<String, u64>,
    pub features: HashMap<String, bool>
}

#[derive(Debug, Deserialize, Clone)]
pub struct Roles {
    pub public: HashMap<String, u64>,
    pub private: HashMap<String, u64>
}

pub fn get_config() -> Result<ConfigData, toml::de::Error>  {
    let data = std::fs::read_to_string("config.toml").expect("Unable to find config.toml file");
    toml::from_str(&data)
}
