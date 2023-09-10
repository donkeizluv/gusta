use crate::client::HikAPI;
use anyhow::{Context, Result};
use rand::{thread_rng, Rng};
use std::time::SystemTime;

pub struct WebEndpoint {
    url: String,
}

impl WebEndpoint {
    pub fn new(url: &str) -> Self {
        WebEndpoint { url: url.into() }
    }
}

impl HikAPI for WebEndpoint {
    fn endpoint(&self) -> &str {
        &self.url
    }

    fn auth_setting_api(&self, username: &str) -> String {
        format!(
            "{}/ISAPI/Security/sessionLogin/capabilities?username={}&random={}",
            self.endpoint(),
            username,
            thread_rng().gen::<u16>()
        )
    }

    fn login_api(&self) -> Result<String> {
        let elapsed = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .context("system time is before EPOCH")?;

        Ok(format!(
            "{}/ISAPI/Security/sessionLogin?timeStamp={}",
            self.endpoint(),
            elapsed.as_secs()
        ))
    }

    fn heartbeat_api(&self) -> String {
        format!("{}/ISAPI/Security/sessionHeartbeat", self.endpoint())
    }

    fn online_users_api(&self) -> String {
        format!("{}/ISAPI/Security/onlineUser", self.endpoint())
    }
}
