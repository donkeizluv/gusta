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
        let from_exe = Self::exe_dir()?;

        let setting_str = match Self::opt_read(from_pwd) {
            Some(s) => Some(s),
            None => Self::opt_read(from_exe),
        };

        match setting_str {
            Some(s) => Ok(toml::from_str(&s)?),
            None => Err(Error::msg("unable to find config file")),
        }
    }
    fn opt_read(path: PathBuf) -> Option<String> {
        match fs::read_to_string(path) {
            Ok(s) => Some(s),
            Err(_) => None,
        }
    }

    fn exe_dir() -> Result<PathBuf> {
        let mut exe = env::current_exe()?;
        exe.pop();
        Ok(exe)
    }
}
