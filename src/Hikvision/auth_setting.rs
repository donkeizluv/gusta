use anyhow::Error;
use serde::{Deserialize, Serialize};
use serde_xml_rs::from_str;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct AuthSetting {
    #[serde(rename = "sessionID")]
    pub session_id: String,

    pub challenge: String,
    pub iterations: u32,
    pub salt: String,

    #[serde(rename = "isIrreversible")]
    pub is_irreversible: bool,

    #[serde(rename = "isSessionIDValidLongTerm")]
    pub is_session_id_valid_long_term: bool,

    #[serde(rename = "sessionIDVersion")]
    pub session_id_version: u32,
}

impl TryFrom<&str> for AuthSetting {
    type Error = Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        Ok(from_str(value)?)
    }
}
