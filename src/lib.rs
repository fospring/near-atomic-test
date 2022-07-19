
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::{UnorderedMap, UnorderedSet},
    env,
    json_types::U128,
    serde::{self, Deserialize, Serialize},
    serde_json, AccountId, Timestamp,
    near_bindgen, BorshStorageKey, PanicOnDefault,
};

#[derive(Debug, BorshStorageKey, BorshSerialize, PartialEq, Eq)]
pub enum StorageKey {
    KeySet,
    Users,
    Infos,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    _key_sets: UnorderedSet<String>,
    users: UnorderedMap<AccountId, String>,
    infos: UnorderedMap<AccountId, u64>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        Self {
            _key_sets: UnorderedSet::new(StorageKey::KeySet),
            users: UnorderedMap::new(StorageKey::Users),
            infos: UnorderedMap::new(StorageKey::Infos),
        }
    }

    pub fn set_key(&mut self, skey: String) {

        self._key_sets.insert(&skey);
    }

    pub fn has_key(&self, skey: String) -> bool {
        self._key_sets.contains(&skey)
    }

    pub fn set_info(&mut self, acc: AccountId, num: u64) {

        self.infos.insert(&acc, &num);
    }

    pub fn get_info(&self, acc: AccountId) -> u64 {
        self.infos.get(&acc).unwrap_or_default()
    }
}
