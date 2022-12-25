use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::{UnorderedMap, UnorderedSet},
    near_bindgen, AccountId, BorshStorageKey, PanicOnDefault,
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

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use std::cmp::Ordering;
    use near_sdk::collections::TreeMap;
    struct Pnl(u128);

    impl PartialEq<Self> for Pnl {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }

    impl Eq for Pnl {}

    impl PartialOrd<Self> for Pnl {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            if self.0 == other.0 {
                Some(Ordering::Equal)
            } else if self.0 > other.0 {
                Some(Ordering::Less)
            } else {
                Some(Ordering::Greater)
            }
        }
    }

    impl Ord for Pnl {
        fn cmp(&self, other: &Self) -> Ordering {
            if self.0 == other.0 {
                Ordering::Equal
            } else if self.0 > other.0 {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        }
    }

    #[test]
    fn test_treemap_order() {

    }
}
