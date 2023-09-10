use anyhow::{Error, Result};
use std::{env, fs, path::PathBuf};

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
        let from_pwd = env::current_dir()?.join(CONFIG_FILENAME);
        let from_exe = Self::exe_dir()?.join(CONFIG_FILENAME);

        let setting_str = match fs::read_to_string(from_pwd).ok() {
            Some(s) => Some(s),
            None => fs::read_to_string(from_exe).ok(),
        };

        match setting_str {
            Some(s) => Ok(toml::from_str(&s)?),
            None => Err(Error::msg("unable to find config file")),
        }
    }

    fn exe_dir() -> Result<PathBuf> {
        let mut exe = env::current_exe()?;
        exe.pop();
        Ok(exe)
    }
}
