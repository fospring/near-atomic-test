use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::UnorderedMap,
    near_bindgen, BorshStorageKey, PanicOnDefault,
};

#[derive(Debug, BorshStorageKey, BorshSerialize, PartialEq, Eq)]
pub enum StorageKey {
    NumInforms,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    count: i32,
    num_infos_storage: UnorderedMap<i32, String>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        Self {
            count: 0,
            num_infos_storage: UnorderedMap::new(StorageKey::NumInforms),
        }
    }

    pub fn set_count(&mut self, count: i32) {
        self.count = count;
    }

    pub fn get_count(&self) -> i32 {
        self.count
    }

    pub fn set_num_info(&mut self, num: i32, info: String) {
        self.num_infos_storage.insert(&num, &info);
    }

    pub fn get_num_info(&self, num: i32) -> Option<String> {
        self.num_infos_storage.get(&num)
    }
}
