use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    str,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct OnlineUserList {
    #[serde(rename = "OnlineUser")]
    pub users: Vec<OnlineUser>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq)]
pub struct OnlineUser {
    pub id: u32,

    pub name: String,

    #[serde(rename = "type")]
    pub user_type: String,

    #[serde(rename = "loginTime")]
    pub login_time: String,

    #[serde(rename = "clientAddress")]
    pub client_address: ClientAddress,
}

impl PartialEq for OnlineUser {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
        // && self.id == other.id
        // && self.user_type == other.user_type
        // && self.login_time == other.login_time
        // && self.client_address == other.client_address
    }
}

impl Hash for OnlineUser {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // self.id.hash(state);
        self.name.hash(state);
        self.user_type.hash(state);
        self.login_time.hash(state);
        self.client_address.hash(state);
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct ClientAddress {
    #[serde(rename = "ipAddress")]
    pub ip_address: String,
    // #[serde(rename = "ipv6Address")]
    // pub ipv6_address: String,
}

impl From<ClientAddress> for String {
    fn from(value: ClientAddress) -> Self {
        value.ip_address
    }
}

pub trait Hashable: Hash {
    fn hash_value(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);

        hasher.finish()
    }
}
impl Hashable for OnlineUser {}
