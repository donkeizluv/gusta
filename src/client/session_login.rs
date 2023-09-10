use anyhow::Error;
use serde::{Deserialize, Serialize};
use serde_xml_rs::from_str;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SessionLogin {
    #[serde(rename = "userName")]
    pub username: String,

    pub password: String,

    #[serde(rename = "sessionID")]
    pub session_id: String,

    #[serde(rename = "isSessionIDValidLongTerm")]
    pub is_session_id_valid_long_term: bool,

    #[serde(rename = "sessionIDVersion")]
    pub session_id_version: u32,
}

impl TryFrom<&str> for SessionLogin {
    type Error = Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        Ok(from_str(value)?)
    }
}
