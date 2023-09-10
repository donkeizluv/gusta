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

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
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

    #[serde(rename = "ipv6Address")]
    pub ipv6_address: String,
}

pub trait Hashable: Hash {
    fn hash_value(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);

        hasher.finish()
    }
}
impl Hashable for OnlineUser {}
