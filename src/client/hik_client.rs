use crate::client::{OnlineUserList, SessionLogin};

use super::auth_setting::*;

use anyhow::{Error, Result};
use reqwest::Client;
use serde_xml_rs::to_string;
use std::time::Duration;
use tokio::{
    task::{self, JoinHandle},
    time,
};

#[derive(Debug)]
enum ClientStatus {
    NotConnected,
    Connected {
        auth_setting: AuthSetting,
        heart_beat_handle: JoinHandle<()>,
        token: String,
    },
}

#[derive(Debug)]
pub struct HikClient<T: HikAPI> {
    pub username: String,
    pub password: String,
    pub api_provider: T,
    connection: ClientStatus,
}

impl<T: HikAPI> HikClient<T> {
    const HB_DELAY: u64 = 10;

    pub fn new(username: &str, password: &str, api_provider: T) -> Self {
        HikClient {
            username: username.into(),
            password: password.into(),
            api_provider,
            connection: ClientStatus::NotConnected,
        }
    }

    #[allow(dead_code)]
    pub fn logout(&mut self) {
        self.disconnect();
    }

    pub async fn login(&mut self) -> Result<()> {
        match self.connection {
            ClientStatus::NotConnected => {
                let client = reqwest::Client::new();
                let setting = self.fetch_auth_setting(&client).await?;

                // login
                let login_payload = SessionLogin {
                    password: self.encoded_pwd(&setting)?,
                    username: self.username.clone(),
                    is_session_id_valid_long_term: setting.is_session_id_valid_long_term,
                    session_id: setting.session_id.clone(),
                    session_id_version: setting.session_id_version,
                };

                let payload_xml = to_string(&login_payload)?;
                let login_res = client
                    .post(self.api_provider.login_api()?)
                    .body(payload_xml) // remove clone when done
                    .send()
                    .await?;

                let auth_token = match login_res.headers().get("Set-Cookie") {
                    Some(t) => utils::extract_cookie(t.to_str()?)?,
                    None => return Err(Error::msg("unable to login")),
                };

                let hb_handle = self.start_hb(&auth_token);

                self.connection = ClientStatus::Connected {
                    auth_setting: setting,
                    heart_beat_handle: hb_handle,
                    token: auth_token,
                };
                Ok(())
            }
            ClientStatus::Connected {
                auth_setting: _,
                heart_beat_handle: _,
                token: _,
            } => Err(Error::msg("already connected")),
        }
    }
    fn start_hb(&self, token: &str) -> JoinHandle<()> {
        let hb_context = HeatbeatContext {
            api: self.api_provider.heartbeat_api(),
            token: token.into(),
        };

        let hb_handle: JoinHandle<()> = task::spawn(async move {
            let mut interval = time::interval(Duration::from_secs(Self::HB_DELAY));
            let HeatbeatContext { api, token } = hb_context;
            let heart_beat_client = reqwest::Client::new();

            loop {
                interval.tick().await;

                let res = heart_beat_client
                    .put(api.clone())
                    .header("Cookie", token.clone())
                    .send()
                    .await;

                match res {
                    Ok(r) => match r.text().await {
                        Ok(_) => {}
                        Err(_) => {
                            println!("unable to read hb response")
                        }
                    },
                    Err(_) => println!("sending hb failed"),
                }
            }
        });

        hb_handle
    }

    // ref script/lib/utils.js
    async fn fetch_auth_setting(&self, rq_client: &Client) -> Result<AuthSetting> {
        let res = rq_client
            .get(self.api_provider.auth_setting_api(&self.username))
            .send()
            .await?;
        let xml = res.text().await?;

        AuthSetting::try_from(xml.as_str())
    }

    pub async fn fetch_online_users(&self) -> Result<OnlineUserList> {
        match &self.connection {
            ClientStatus::NotConnected => Err(Error::msg("not logged in")),
            ClientStatus::Connected {
                auth_setting: _,
                heart_beat_handle: _,
                token,
            } => {
                let client = reqwest::Client::new();
                let res = client
                    .get(self.api_provider.online_users_api())
                    .header("Cookie", token)
                    .send()
                    .await?;
                let body = res.text().await?;
                match serde_xml_rs::from_str::<OnlineUserList>(&body) {
                    Ok(users) => Ok(users),
                    Err(_) => Err(Error::msg("unable to deserialize result")),
                }
            }
        }
    }

    fn encoded_pwd(&self, setting: &AuthSetting) -> Result<String> {
        if setting.is_irreversible {
            let cred_hash = sha256::digest(
                [
                    self.username.clone(),
                    setting.salt.clone(),
                    self.password.clone(),
                ]
                .join(""),
            );
            let mut result = sha256::digest([cred_hash, setting.challenge.clone()].join(""));

            for _ in 2..setting.iterations {
                result = sha256::digest(result);
            }

            return Ok(result);
        }

        let mut result = [
            sha256::digest(self.password.clone()),
            setting.challenge.clone(),
        ]
        .join("");

        for _ in 1..setting.iterations {
            result = sha256::digest(result);
        }

        Ok(result)
    }

    fn disconnect(&mut self) {
        match &self.connection {
            ClientStatus::NotConnected => (),
            ClientStatus::Connected {
                auth_setting: _,
                heart_beat_handle,
                token: _,
            } => {
                heart_beat_handle.abort();
                self.connection = ClientStatus::NotConnected
            }
        }
    }
}

struct HeatbeatContext {
    api: String,
    token: String,
}

impl<T: HikAPI> Drop for HikClient<T> {
    fn drop(&mut self) {
        self.disconnect()
    }
}

pub trait HikAPI {
    fn endpoint(&self) -> &str;
    fn auth_setting_api(&self, username: &str) -> String;
    fn login_api(&self) -> Result<String>;
    fn heartbeat_api(&self) -> String;
    fn online_users_api(&self) -> String;
}

mod utils {
    use anyhow::{Error, Result};

    pub fn extract_cookie(value: &str) -> Result<String> {
        let splited: Vec<String> = value.split(';').map(|s| s.to_string()).collect();
        let token = splited
            .first()
            .ok_or(Error::msg("unable to get auth token"))?;

        Ok(token.clone())
    }
}
