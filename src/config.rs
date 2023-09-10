use anyhow::Result;
use std::{env, fs};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub endpoint: String,
    pub username: String,
    pub password: String,
}
const CONFIG_FILENAME: &str = "Config.toml";

impl Config {
    pub fn read_env() -> Result<Self> {
        let env = env::current_dir()?.join(CONFIG_FILENAME);
        let setting_str = match fs::read_to_string(env) {
            Ok(s) => s,
            Err(_) => panic!("missing config file"),
        };
        let setting: Self = toml::from_str(&setting_str)?;

        Ok(setting)
    }
}
